// Per-thread observation of the Nimble process via /proc/<pid>/task/*.

use serde::Serialize;
use std::collections::{HashMap, VecDeque};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
struct Raw {
    name: String,
    state: char,
    ticks: u64,
    processor: i64,
    wchan: String,
    syscall: String,
    vcsw: u64,
    nvcsw: u64,
}

#[derive(Clone, Serialize)]
pub struct View {
    pub tid: i32,
    pub name: String,
    pub state: char,
    pub cpu_pct: f64,
    pub cpu: i64,
    pub wchan: String,
    pub syscall: String,
    pub vcsw_d: u64,
    pub nvcsw_d: u64,
}

#[derive(Clone, Serialize)]
pub struct SecView {
    pub tid: i32,
    pub name: String,
    pub cpu_pct: f64,
    pub states: String,
    pub last_wchan: String,
    pub last_syscall: String,
    pub vcsw_d: u64,
    pub nvcsw_d: u64,
}

pub struct ThreadMon {
    pub pid: i32,
    pub ring: Mutex<VecDeque<(u64, Vec<View>)>>,
    pub last_second: Mutex<Vec<SecView>>,
    pub thread_count: Mutex<usize>,
}

fn clk_tck() -> f64 {
    let v = unsafe { libc::sysconf(libc::_SC_CLK_TCK) };
    if v > 0 { v as f64 } else { 100.0 }
}

pub fn find_nimble() -> Option<i32> {
    let rd = std::fs::read_dir("/proc").ok()?;
    for e in rd.flatten() {
        let pid: i32 = match e.file_name().to_str().and_then(|s| s.parse().ok()) {
            Some(p) => p,
            None => continue,
        };
        let comm = std::fs::read_to_string(format!("/proc/{}/comm", pid)).unwrap_or_default();
        if comm.trim() == "nimble" {
            return Some(pid);
        }
    }
    None
}

fn read_task(pid: i32, tid: i32) -> Option<Raw> {
    let base = format!("/proc/{}/task/{}", pid, tid);
    let stat = std::fs::read_to_string(format!("{}/stat", base)).ok()?;
    let lp = stat.find('(')?;
    let rp = stat.rfind(')')?;
    let name = stat[lp + 1..rp].to_string();
    let f: Vec<&str> = stat[rp + 1..].split_whitespace().collect();
    // f[0]=state(3) ... f[11]=utime(14) f[12]=stime(15) ... f[36]=processor(39)
    let state = f.first()?.chars().next().unwrap_or('?');
    let ut: u64 = f.get(11)?.parse().ok()?;
    let st: u64 = f.get(12)?.parse().ok()?;
    let processor = f.get(36).and_then(|s| s.parse().ok()).unwrap_or(-1);
    let wchan = std::fs::read_to_string(format!("{}/wchan", base)).unwrap_or_default().trim().to_string();
    let syscall = std::fs::read_to_string(format!("{}/syscall", base))
        .ok()
        .and_then(|s| s.split_whitespace().next().map(|x| x.to_string()))
        .unwrap_or_default();
    let status = std::fs::read_to_string(format!("{}/status", base)).unwrap_or_default();
    let mut vcsw = 0;
    let mut nvcsw = 0;
    for l in status.lines() {
        if let Some(v) = l.strip_prefix("voluntary_ctxt_switches:") {
            vcsw = v.trim().parse().unwrap_or(0);
        } else if let Some(v) = l.strip_prefix("nonvoluntary_ctxt_switches:") {
            nvcsw = v.trim().parse().unwrap_or(0);
        }
    }
    Some(Raw { name, state, ticks: ut + st, processor, wchan, syscall, vcsw, nvcsw })
}

fn sample(pid: i32) -> HashMap<i32, Raw> {
    let mut m = HashMap::new();
    if let Ok(rd) = std::fs::read_dir(format!("/proc/{}/task", pid)) {
        for e in rd.flatten() {
            if let Some(tid) = e.file_name().to_str().and_then(|s| s.parse::<i32>().ok()) {
                if let Some(r) = read_task(pid, tid) {
                    m.insert(tid, r);
                }
            }
        }
    }
    m
}

pub fn is_interesting(name: &str) -> bool {
    let n = name.to_ascii_lowercase();
    ["mpeg2ts", "udp", "camera", "mcast", "multicast", "work", "m2ts"].iter().any(|k| n.contains(k))
}

impl ThreadMon {
    pub fn new(pid: i32) -> Self {
        ThreadMon {
            pid,
            ring: Mutex::new(VecDeque::new()),
            last_second: Mutex::new(Vec::new()),
            thread_count: Mutex::new(0),
        }
    }

    pub fn run(self: Arc<Self>, interval_ms: u64, keep_ns: u64, stop: &AtomicBool) {
        let tck = clk_tck();
        let mut prev = sample(self.pid);
        let mut prev_t = std::time::Instant::now();
        let mut sec_base = prev.clone();
        let mut sec_t = prev_t;
        let mut states: HashMap<i32, String> = HashMap::new();
        while !stop.load(Ordering::Relaxed) {
            std::thread::sleep(std::time::Duration::from_millis(interval_ms));
            let cur = sample(self.pid);
            let now = std::time::Instant::now();
            let dt = now.duration_since(prev_t).as_secs_f64().max(1e-3);
            let ts = crate::util::now_ns();
            let mut views = Vec::with_capacity(cur.len());
            for (tid, r) in &cur {
                let p = prev.get(tid);
                let dticks = p.map(|p| r.ticks.saturating_sub(p.ticks)).unwrap_or(0);
                let st = states.entry(*tid).or_default();
                if st.len() < 64 {
                    st.push(r.state);
                }
                views.push(View {
                    tid: *tid,
                    name: r.name.clone(),
                    state: r.state,
                    cpu_pct: dticks as f64 / tck / dt * 100.0,
                    cpu: r.processor,
                    wchan: r.wchan.clone(),
                    syscall: r.syscall.clone(),
                    vcsw_d: p.map(|p| r.vcsw.saturating_sub(p.vcsw)).unwrap_or(0),
                    nvcsw_d: p.map(|p| r.nvcsw.saturating_sub(p.nvcsw)).unwrap_or(0),
                });
            }
            if let Ok(mut ring) = self.ring.lock() {
                ring.push_back((ts, views));
                while let Some((t0, _)) = ring.front() {
                    if ts.saturating_sub(*t0) > keep_ns { ring.pop_front(); } else { break; }
                }
            }
            if let Ok(mut c) = self.thread_count.lock() {
                *c = cur.len();
            }
            // one-second rollup
            let sdt = now.duration_since(sec_t).as_secs_f64();
            if sdt >= 1.0 {
                let mut out = Vec::new();
                for (tid, r) in &cur {
                    let b = sec_base.get(tid);
                    let dticks = b.map(|b| r.ticks.saturating_sub(b.ticks)).unwrap_or(0);
                    let cpu = dticks as f64 / tck / sdt * 100.0;
                    let st = states.get(tid).cloned().unwrap_or_default();
                    if is_interesting(&r.name) || cpu >= 5.0 || st.contains('D') {
                        out.push(SecView {
                            tid: *tid,
                            name: r.name.clone(),
                            cpu_pct: (cpu * 10.0).round() / 10.0,
                            states: st,
                            last_wchan: r.wchan.clone(),
                            last_syscall: r.syscall.clone(),
                            vcsw_d: b.map(|b| r.vcsw.saturating_sub(b.vcsw)).unwrap_or(0),
                            nvcsw_d: b.map(|b| r.nvcsw.saturating_sub(b.nvcsw)).unwrap_or(0),
                        });
                    }
                }
                out.sort_by(|a, b| b.cpu_pct.partial_cmp(&a.cpu_pct).unwrap_or(std::cmp::Ordering::Equal));
                if let Ok(mut ls) = self.last_second.lock() {
                    *ls = out;
                }
                states.clear();
                sec_base = cur.clone();
                sec_t = now;
            }
            prev = cur;
            prev_t = now;
        }
    }

    pub fn ring_window(&self, from: u64, to: u64) -> Vec<(u64, Vec<View>)> {
        self.ring
            .lock()
            .map(|r| r.iter().filter(|(t, _)| *t >= from && *t <= to).cloned().collect())
            .unwrap_or_default()
    }
}
