// nimdiag: read-only multicast ingest diagnostics for Nimble Streamer hosts,
// plus an offline analyzer for the blocked-resolver ingest stall (stall.rs).

mod wire;
mod capture;
mod config;
mod env;
mod kernel;
mod recorder;
mod report;
mod stall;
mod threads;
mod util;

use wire::{Group, SecAcc};
use capture::{PacketSocket, Shared};
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::net::Ipv4Addr;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use util::*;

static STOP: AtomicBool = AtomicBool::new(false);

extern "C" fn on_signal(_: libc::c_int) {
    STOP.store(true, Ordering::SeqCst);
}

pub struct Opts {
    pub config: String,
    pub duration: u64,
    pub out: String,
    pub pre: u64,
    pub post: u64,
    pub ring_mb: usize,
    pub trigger_rxq: u64,
    pub trigger_cc: bool,
    pub cooldown: u64,
    pub max_events: u64,
    pub max_disk_mb: u64,
    pub event_pcap_all: bool,
    pub pid: Option<i32>,
    pub join: bool,
    pub thread_ms: u64,
    pub console: u64,
    pub iface: Option<String>,
    pub mip: Option<Ipv4Addr>,
    pub rcvbuf_mb: i32,
    pub tar: bool,
    pub dry_run: bool,
    pub nimble_conf: String,
}

const TOP_HELP: &str = "nimdiag - read-only MPEG-TS ingest diagnostics for Nimble Streamer

USAGE:
  nimdiag [collect] --config streams.json [options]   record a live run (root, see collect --help)
  nimdiag analyze DIR [--stream NAME] [--dns FILE] [--strace FILE] [--nimble-log FILE] [--json]
                                                      check a result directory for the blocked-resolver stall
  nimdiag udp FILE [--json]                           decode a /proc/net/udp copy

analyze and udp only read files; they need no privileges and can run on any machine.
";

const HELP: &str = "nimdiag collect - record a live diagnostic run

USAGE:
  nimdiag [collect] --config streams.json [options]

The config is Nimble's stream list JSON (UDP multicast entries with ip/port/mip are used).
Run as root. Nothing in Nimble is modified.

OPTIONS:
  --config FILE          Nimble streams JSON (required)
  --duration SEC         Run time, default 1800 (Ctrl-C stops early and still writes results)
  --out DIR              Output parent directory, default .
  --pre SEC              Event window before trigger, default 10
  --post SEC             Event window after trigger, default 5
  --ring-mb N            Per-stream packet ring for event pcaps, MiB, default 64 (0 = no pcaps)
  --trigger-rxq BYTES    Trigger when a Nimble socket rx_queue >= BYTES, default 1048576
  --trigger-cc           Also trigger on CC errors seen on the wire
  --cooldown SEC         Min time between events of one stream, default 120
  --max-events N         Max events in total, default 30
  --max-disk-mb N        Stop writing pcaps after N MiB, default 2048
  --event-pcap-all       Event pcap contains all streams, not only the triggering one
  --pid PID              Nimble PID (default: process with comm 'nimble')
  --no-join              Do not hold IGMP membership for configured groups
  --thread-ms MS         Thread sampling interval, default 200
  --console SEC          Console table interval, default 5
  --iface NAME           Capture interface (default: resolved from 'mip')
  --mip IP               Default interface address for streams without 'mip'
  --rcvbuf-mb N          Capture socket buffer, MiB, default 64
  --no-tar               Do not create the .tar.gz at the end
  --dry-run              Parse config, resolve interfaces, print plan and exit
  --nimble-conf FILE     Nimble config copied (masked) into env/, default /etc/nimble/nimble.conf
  -h, --help             This help
";

fn parse_args(args: Vec<String>) -> Result<Opts, String> {
    let mut o = Opts {
        config: String::new(),
        duration: 1800,
        out: ".".into(),
        pre: 10,
        post: 5,
        ring_mb: 64,
        trigger_rxq: 1_048_576,
        trigger_cc: false,
        cooldown: 120,
        max_events: 30,
        max_disk_mb: 2048,
        event_pcap_all: false,
        pid: None,
        join: true,
        thread_ms: 200,
        console: 5,
        iface: None,
        mip: None,
        rcvbuf_mb: 64,
        tar: true,
        dry_run: false,
        nimble_conf: "/etc/nimble/nimble.conf".into(),
    };
    let mut i = 0;
    fn val<'a>(args: &'a [String], i: &mut usize, name: &str) -> Result<&'a str, String> {
        *i += 1;
        args.get(*i).map(|s| s.as_str()).ok_or_else(|| format!("{} needs a value", name))
    }
    fn num<T: std::str::FromStr>(s: &str, name: &str) -> Result<T, String> {
        s.parse().map_err(|_| format!("bad value for {}: {}", name, s))
    }
    while i < args.len() {
        let a = args[i].as_str();
        match a {
            "-h" | "--help" => {
                print!("{}", HELP);
                std::process::exit(0);
            }
            "--config" => o.config = val(&args, &mut i, a)?.to_string(),
            "--duration" => o.duration = num(val(&args, &mut i, a)?, a)?,
            "--out" => o.out = val(&args, &mut i, a)?.to_string(),
            "--pre" => o.pre = num(val(&args, &mut i, a)?, a)?,
            "--post" => o.post = num(val(&args, &mut i, a)?, a)?,
            "--ring-mb" => o.ring_mb = num(val(&args, &mut i, a)?, a)?,
            "--trigger-rxq" => o.trigger_rxq = num(val(&args, &mut i, a)?, a)?,
            "--trigger-cc" => o.trigger_cc = true,
            "--cooldown" => o.cooldown = num(val(&args, &mut i, a)?, a)?,
            "--max-events" => o.max_events = num(val(&args, &mut i, a)?, a)?,
            "--max-disk-mb" => o.max_disk_mb = num(val(&args, &mut i, a)?, a)?,
            "--event-pcap-all" => o.event_pcap_all = true,
            "--pid" => o.pid = Some(num(val(&args, &mut i, a)?, a)?),
            "--no-join" => o.join = false,
            "--thread-ms" => o.thread_ms = num::<u64>(val(&args, &mut i, a)?, a)?.max(50),
            "--console" => o.console = num::<u64>(val(&args, &mut i, a)?, a)?.max(1),
            "--iface" => o.iface = Some(val(&args, &mut i, a)?.to_string()),
            "--mip" => o.mip = Some(num(val(&args, &mut i, a)?, a)?),
            "--rcvbuf-mb" => o.rcvbuf_mb = num(val(&args, &mut i, a)?, a)?,
            "--no-tar" => o.tar = false,
            "--dry-run" => o.dry_run = true,
            "--nimble-conf" => o.nimble_conf = val(&args, &mut i, a)?.to_string(),
            other => return Err(format!("unknown option: {} (see --help)", other)),
        }
        i += 1;
    }
    if o.config.is_empty() {
        return Err("--config is required (see --help)".into());
    }
    Ok(o)
}

struct Pending {
    id: u64,
    group: usize,
    t0: u64,
    due: u64,
    reason: String,
    snapshot: Value,
}

struct Log {
    file: File,
}

impl Log {
    fn line(&mut self, s: &str) {
        println!("{}", s);
        let _ = writeln!(self.file, "{}", s);
    }
}

fn jsonl(path: &Path) -> File {
    OpenOptions::new().create(true).append(true).open(path).expect("cannot open output file")
}

fn top_sizes(sizes: &std::collections::BTreeMap<u32, u64>, n: usize) -> Vec<(u32, u64)> {
    let mut v: Vec<(u32, u64)> = sizes.iter().map(|(k, v)| (*k, *v)).collect();
    v.sort_by(|a, b| b.1.cmp(&a.1));
    v.truncate(n);
    v
}

fn sec_json(t: u64, g: &Group, s: &SecAcc, sock: &Value) -> Value {
    let sizes: Value = top_sizes(&s.sizes, 6)
        .iter()
        .map(|(k, v)| json!([k, v]))
        .collect::<Vec<_>>()
        .into();
    json!({
        "t": fmt_utc_ms(t),
        "type": "stream",
        "name": g.cfg.name,
        "group": g.cfg.label(),
        "pkts": s.pkts,
        "mbps": (s.bytes as f64 * 8.0 / 1e6 * 1000.0).round() / 1000.0,
        "sizes": sizes,
        "burst_1ms": s.burst_1ms,
        "burst_10ms": s.burst_10ms,
        "max_gap_ms": (s.max_gap_ns as f64 / 1e6 * 10.0).round() / 10.0,
        "srcs": s.srcs.iter().map(|x| ip_from_key(*x).to_string()).collect::<Vec<_>>(),
        "ttl": [s.ttl_min, s.ttl_max],
        "frags": s.frags,
        "rtp": s.rtp_pkts,
        "rtp_seq_err": s.rtp_seq_err,
        "ts_pkts": s.ts_pkts,
        "non188": s.non188,
        "sync_err": s.sync_err,
        "cc_err": s.cc_err,
        "cc_dup": s.cc_dup,
        "tei": s.tei,
        "scrambled": s.scrambled,
        "null_pct": if s.ts_pkts > 0 { (s.null_pkts as f64 * 1000.0 / s.ts_pkts as f64).round() / 10.0 } else { 0.0 },
        "pcr_jitter_max_ms": (s.pcr_jitter_max_ns as f64 / 1e6 * 100.0).round() / 100.0,
        "pcr_mbps": if s.pcr_br_n > 0 { (s.pcr_br_sum / s.pcr_br_n as f64 / 1e6 * 1000.0).round() / 1000.0 } else { 0.0 },
        "pcr_disc": s.pcr_disc,
        "pids": s.pids.len(),
        "sock": sock,
    })
}

fn delta_map(cur: &HashMap<String, u64>, prev: &HashMap<String, u64>) -> Value {
    let mut m = serde_json::Map::new();
    let mut keys: Vec<&String> = cur.keys().collect();
    keys.sort();
    for k in keys {
        let d = cur[k].saturating_sub(*prev.get(k).unwrap_or(&cur[k]));
        m.insert(k.clone(), json!(d));
    }
    Value::Object(m)
}

fn sleep_until(t: Instant) {
    while Instant::now() < t {
        if STOP.load(Ordering::Relaxed) {
            return;
        }
        let left = t.saturating_duration_since(Instant::now());
        std::thread::sleep(left.min(Duration::from_millis(100)));
    }
}

fn run_analyze(args: &[String]) -> i32 {
    let mut dir = None;
    let (mut stream, mut dns, mut strace, mut log, mut as_json) = (None, None, None, None, false);
    let mut i = 0;
    while i < args.len() {
        let a = args[i].as_str();
        let mut next = || {
            i += 1;
            args.get(i).cloned().ok_or_else(|| format!("{} needs a value", a))
        };
        let r = match a {
            "--stream" => next().map(|v| stream = Some(v)),
            "--dns" => next().map(|v| dns = Some(PathBuf::from(v))),
            "--strace" => next().map(|v| strace = Some(PathBuf::from(v))),
            "--nimble-log" => next().map(|v| log = Some(PathBuf::from(v))),
            "--json" => Ok(as_json = true),
            "-h" | "--help" => {
                print!("{}", TOP_HELP);
                return 0;
            }
            x if !x.starts_with('-') && dir.is_none() => Ok(dir = Some(PathBuf::from(x))),
            x => Err(format!("unknown argument: {}", x)),
        };
        if let Err(e) = r {
            eprintln!("error: {}", e);
            return 2;
        }
        i += 1;
    }
    let dir = match dir {
        Some(d) => d,
        None => {
            eprintln!("error: analyze needs a nimdiag result directory");
            return 2;
        }
    };
    let inp = stall::Inputs {
        dir: &dir,
        stream: stream.as_deref(),
        dns: dns.as_deref(),
        strace: strace.as_deref(),
        nimble_log: log.as_deref(),
    };
    match stall::analyze(&inp) {
        Ok(r) => {
            if as_json {
                println!("{}", serde_json::to_string_pretty(&r).unwrap_or_default());
            } else {
                print!("{}", stall::report_text(&r));
            }
            0
        }
        Err(e) => {
            eprintln!("error: {}", e);
            2
        }
    }
}

fn run_udp(args: &[String]) -> i32 {
    let as_json = args.iter().any(|a| a == "--json");
    let files: Vec<&String> = args.iter().filter(|a| !a.starts_with('-')).collect();
    if files.len() != 1 {
        eprintln!("usage: nimdiag udp FILE [--json]");
        return 2;
    }
    match stall::udp_report(Path::new(files[0])) {
        Ok((v, t)) => {
            if as_json {
                println!("{}", serde_json::to_string_pretty(&v).unwrap_or_default());
            } else {
                print!("{}", t);
            }
            0
        }
        Err(e) => {
            eprintln!("error: {}", e);
            2
        }
    }
}

fn main() {
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(|s| s.as_str()) {
        Some("analyze") => std::process::exit(run_analyze(&args[1..])),
        Some("udp") => std::process::exit(run_udp(&args[1..])),
        Some("collect") => {
            args.remove(0);
        }
        Some("-h") | Some("--help") | None => {
            print!("{}", TOP_HELP);
            std::process::exit(if args.is_empty() { 2 } else { 0 });
        }
        _ => {}
    }
    collect(args);
}

fn collect(args: Vec<String>) {
    let opts = match parse_args(args) {
        Ok(o) => o,
        Err(e) => {
            eprintln!("error: {}", e);
            std::process::exit(2);
        }
    };
    let (streams, total) = match config::load(&opts.config) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("error: {}", e);
            std::process::exit(2);
        }
    };
    if streams.is_empty() {
        eprintln!("error: no UDP multicast streams found in {} ({} entries total)", opts.config, total);
        std::process::exit(2);
    }

    // resolve capture interfaces
    let mut ifaces: Vec<String> = Vec::new();
    let mut plan = Vec::new();
    for s in &streams {
        let mip = s.mip.or(opts.mip);
        let iface = match (&opts.iface, mip) {
            (Some(i), _) => Some(i.clone()),
            (None, Some(m)) => capture::iface_for_ip(m),
            (None, None) => None,
        };
        match iface {
            Some(i) => {
                if !ifaces.contains(&i) {
                    ifaces.push(i.clone());
                }
                plan.push(format!("  {:<48} {:<22} mip={:<15} iface={}", s.name, s.label(),
                    mip.map(|m| m.to_string()).unwrap_or("-".into()), i));
            }
            None => {
                eprintln!("error: cannot resolve interface for {} ({}), mip={:?}. Use --iface or --mip.",
                    s.name, s.label(), mip);
                std::process::exit(2);
            }
        }
    }
    let nimble_pid = opts.pid.or_else(threads::find_nimble);
    println!("nimdiag {} | {} UDP multicast streams of {} entries | ifaces: {} | nimble pid: {}",
        env!("CARGO_PKG_VERSION"), streams.len(), total, ifaces.join(","),
        nimble_pid.map(|p| p.to_string()).unwrap_or_else(|| "not found".into()));
    for p in &plan {
        println!("{}", p);
    }
    if opts.dry_run {
        return;
    }
    if unsafe { libc::geteuid() } != 0 {
        eprintln!("error: must run as root (AF_PACKET capture and /proc access)");
        std::process::exit(2);
    }

    unsafe {
        libc::signal(libc::SIGINT, on_signal as *const () as libc::sighandler_t);
        libc::signal(libc::SIGTERM, on_signal as *const () as libc::sighandler_t);
    }

    let start = now_ns();
    let dir_name = format!("nimdiag_{}_{}", sanitize(&hostname()), fmt_compact(start));
    let out: PathBuf = Path::new(&opts.out).join(&dir_name);
    fs::create_dir_all(out.join("events")).expect("cannot create output directory");
    let _ = fs::copy(&opts.config, out.join("streams.json"));
    let mut log = Log { file: jsonl(&out.join("console.log")) };
    log.line(&format!("{} start, output: {}", fmt_utc(start), out.display()));
    log.line("collecting environment snapshot...");
    env::snapshot(&out.join("env"), &ifaces, nimble_pid, &opts.nimble_conf);

    // shared state
    let keep_ns = (opts.pre + opts.post + 3) * 1_000_000_000;
    let ring_bytes = opts.ring_mb * 1024 * 1024;
    let mut index = HashMap::new();
    let mut groups = Vec::new();
    for s in &streams {
        if index.contains_key(&s.key()) {
            continue;
        }
        index.insert(s.key(), groups.len());
        groups.push(Arc::new(Mutex::new(Group::new(s.clone(), ring_bytes, keep_ns))));
    }
    let shared = Arc::new(Shared {
        groups,
        index,
        other: Mutex::new(HashMap::new()),
        cap_packets: Default::default(),
        cap_drops: Default::default(),
        stop: AtomicBool::new(false),
    });

    // IGMP membership holders
    let mut join_fds = Vec::new();
    if opts.join {
        for s in &streams {
            if let Some(m) = s.mip.or(opts.mip) {
                match capture::join_only(s.group, m) {
                    Ok(fd) => join_fds.push(fd),
                    Err(e) => log.line(&format!("warn: IGMP join {} via {} failed: {}", s.group, m, e)),
                }
            }
        }
    }

    // capture threads
    let mut handles = Vec::new();
    for i in &ifaces {
        let ps = match PacketSocket::open(i, opts.rcvbuf_mb.saturating_mul(1024 * 1024)) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("error: cannot open capture on {}: {}", i, e);
                std::process::exit(1);
            }
        };
        log.line(&format!("capture on {} (socket buffer {} KiB)", i, ps.rcvbuf / 1024));
        let sh = shared.clone();
        handles.push(std::thread::spawn(move || ps.run(&sh)));
    }

    // thread monitor
    let tmon = nimble_pid.map(|p| Arc::new(threads::ThreadMon::new(p)));
    if let Some(tm) = &tmon {
        let tm = tm.clone();
        let sh = shared.clone();
        let ms = opts.thread_ms;
        handles.push(std::thread::spawn(move || tm.run(ms, keep_ns, &sh.stop)));
    } else {
        log.line("warn: Nimble process not found, thread monitoring disabled (use --pid)");
    }

    let mut f_timeline = jsonl(&out.join("timeline.jsonl"));
    let mut f_threads = jsonl(&out.join("threads.jsonl"));
    let mut f_events = jsonl(&out.join("events.jsonl"));

    let n = shared.groups.len();
    let mut prev_drops: HashMap<u64, u64> = HashMap::new();
    let mut first_round = true;
    let mut owners: HashMap<u64, (i32, String)> = HashMap::new();
    let mut owners_at = 0u64;
    let mut history: Vec<VecDeque<(u64, Value)>> = vec![VecDeque::new(); n];
    let mut last_event: Vec<u64> = vec![0; n];
    let mut pending: Vec<Pending> = Vec::new();
    let mut event_id = 0u64;
    let mut disk_bytes = 0u64;
    let mut prev_snmp = kernel::read_snmp_udp();
    let snmp_start = prev_snmp.clone();
    let mut prev_soft = kernel::read_softnet();
    let soft_start = prev_soft;
    let mut prev_nic: HashMap<String, HashMap<String, u64>> =
        ifaces.iter().map(|i| (i.clone(), kernel::read_nic(i))).collect();
    let nic_start = prev_nic.clone();
    let mut prev_cap = (0u64, 0u64);
    // console accumulators: (pkts, bytes, drops, cc, rxq_max, secs)
    let mut con: Vec<(u64, u64, u64, u64, u64, u64)> = vec![(0, 0, 0, 0, 0, 0); n];
    let mut con_tick = 0u64;
    let end = start + opts.duration * 1_000_000_000;
    let mut next = Instant::now() + Duration::from_secs(1);
    log.line(&format!("running for {} s (Ctrl-C to stop early)...", opts.duration));

    loop {
        sleep_until(next);
        next += Duration::from_secs(1);
        let t = now_ns();
        let udp = kernel::read_udp();

        // refresh socket owners every 10 s
        let group_inodes: Vec<u64> = udp
            .iter()
            .filter(|u| shared.index.contains_key(&(u.ip, u.port)))
            .map(|u| u.inode)
            .collect();
        if t.saturating_sub(owners_at) > 10_000_000_000 || group_inodes.iter().any(|i| !owners.contains_key(i)) {
            owners = kernel::inode_owners(&group_inodes, nimble_pid);
            owners_at = t;
        }

        let mut at_trigger_state = Vec::with_capacity(n);
        let mut new_triggers: Vec<(usize, String)> = Vec::new();
        for gi in 0..n {
            let (sec, rec) = {
                let mut g = shared.groups[gi].lock().unwrap();
                let sec = g.take_second();
                let (gip, gport) = g.cfg.key();
                let socks: Vec<&kernel::UdpSock> = udp.iter().filter(|u| u.ip == gip && u.port == gport).collect();
                let rxq = socks.iter().map(|s| s.rxq).max().unwrap_or(0);
                let mut dd = 0u64;
                let mut dt = 0u64;
                let mut own = Vec::new();
                for s in &socks {
                    dt += s.drops;
                    let p = prev_drops.get(&s.inode).copied();
                    dd += match p {
                        Some(p) => s.drops.saturating_sub(p),
                        None if first_round => 0,
                        None => s.drops,
                    };
                    if let Some((pid, c)) = owners.get(&s.inode) {
                        own.push(format!("{}:{}", c, pid));
                    }
                }
                let tot = &mut g.total;
                if !socks.is_empty() {
                    tot.sock_seen = true;
                }
                tot.rxq_max = tot.rxq_max.max(rxq);
                tot.drops_total += dd;
                if dd > 0 {
                    tot.seconds_with_drops += 1;
                }
                let sock = json!({
                    "count": socks.len(),
                    "rxq": rxq,
                    "drops_d": dd,
                    "drops_total": dt,
                    "inodes": socks.iter().map(|s| s.inode).collect::<Vec<_>>(),
                    "owners": own,
                });
                at_trigger_state.push(json!({"name": g.cfg.name, "group": g.cfg.label(), "rxq": rxq, "drops_d": dd,
                    "wire_mbps": (sec.bytes as f64 * 8.0 / 1e6 * 1000.0).round() / 1000.0}));
                let rec = sec_json(t, &g, &sec, &sock);
                // trigger evaluation
                let mut reason = Vec::new();
                if dd > 0 {
                    reason.push(format!("socket drops +{}", dd));
                }
                if rxq >= opts.trigger_rxq {
                    reason.push(format!("rx_queue {} >= {}", rxq, opts.trigger_rxq));
                }
                if opts.trigger_cc && sec.cc_err > 0 {
                    reason.push(format!("wire CC errors {}", sec.cc_err));
                }
                if !reason.is_empty()
                    && event_id < opts.max_events
                    && t.saturating_sub(last_event[gi]) >= opts.cooldown * 1_000_000_000
                {
                    new_triggers.push((gi, reason.join(", ")));
                }
                let c = &mut con[gi];
                c.0 += sec.pkts;
                c.1 += sec.bytes;
                c.2 += dd;
                c.3 += sec.cc_err;
                c.4 = c.4.max(rxq);
                c.5 += 1;
                (sec, rec)
            };
            let _ = sec;
            let _ = writeln!(f_timeline, "{}", rec);
            history[gi].push_back((t, rec));
            while let Some((t0, _)) = history[gi].front() {
                if t.saturating_sub(*t0) > keep_ns { history[gi].pop_front(); } else { break; }
            }
        }
        for s in &udp {
            prev_drops.insert(s.inode, s.drops);
        }
        first_round = false;

        // system-wide counters
        let snmp = kernel::read_snmp_udp();
        let soft = kernel::read_softnet();
        let cap = (shared.cap_packets.load(Ordering::Relaxed), shared.cap_drops.load(Ordering::Relaxed));
        let mut nic_d = serde_json::Map::new();
        for i in &ifaces {
            let cur = kernel::read_nic(i);
            nic_d.insert(i.clone(), delta_map(&cur, &prev_nic[i]));
            prev_nic.insert(i.clone(), cur);
        }
        let sys = json!({
            "t": fmt_utc_ms(t),
            "type": "sys",
            "udp": delta_map(&snmp, &prev_snmp),
            "softnet_dropped_d": soft.0.saturating_sub(prev_soft.0),
            "softnet_squeeze_d": soft.1.saturating_sub(prev_soft.1),
            "nic": nic_d,
            "capture_pkts_d": cap.0.saturating_sub(prev_cap.0),
            "capture_drops_d": cap.1.saturating_sub(prev_cap.1),
            "nimble_threads": tmon.as_ref().and_then(|m| m.thread_count.lock().ok().map(|c| *c)),
            "dns_socks": kernel::dns_sockets(&udp, nimble_pid),
        });
        let _ = writeln!(f_timeline, "{}", sys);
        let rcvbuf_err_d = snmp.get("RcvbufErrors").copied().unwrap_or(0)
            .saturating_sub(prev_snmp.get("RcvbufErrors").copied().unwrap_or(0));
        let cap_drop_d = cap.1.saturating_sub(prev_cap.1);
        prev_snmp = snmp;
        prev_soft = soft;
        prev_cap = cap;

        if let Some(tm) = &tmon {
            if let Ok(ls) = tm.last_second.lock() {
                let _ = writeln!(f_threads, "{}", json!({"t": fmt_utc_ms(t), "threads": *ls}));
            }
        }

        // new events
        for (gi, reason) in new_triggers {
            event_id += 1;
            last_event[gi] = t;
            let name = shared.groups[gi].lock().unwrap().cfg.name.clone();
            shared.groups[gi].lock().unwrap().total.events += 1;
            log.line(&format!("{} EVENT #{} {} : {} (collecting {} s post window)",
                fmt_utc(t), event_id, name, reason, opts.post));
            pending.push(Pending {
                id: event_id,
                group: gi,
                t0: t,
                due: t + opts.post * 1_000_000_000,
                reason,
                snapshot: json!({"streams": at_trigger_state.clone(), "sys": sys.clone()}),
            });
        }

        let stopping = STOP.load(Ordering::Relaxed) || t >= end;

        // finish due events
        let mut i = 0;
        while i < pending.len() {
            if pending[i].due <= t || stopping {
                let p = pending.remove(i);
                finish_event(&p, &shared, &history, tmon.as_deref(), &opts, &out, &mut disk_bytes, &mut f_events, &mut log);
            } else {
                i += 1;
            }
        }

        // console
        con_tick += 1;
        if con_tick % opts.console == 0 || stopping {
            print_console(&mut log, t, &shared, &mut con, cap_drop_d, rcvbuf_err_d, nimble_pid);
        }
        if stopping {
            break;
        }
    }

    shared.stop.store(true, Ordering::SeqCst);
    for h in handles {
        let _ = h.join();
    }
    for fd in join_fds {
        unsafe { libc::close(fd) };
    }
    let t_end = now_ns();
    let _ = fs::copy("/proc/net/udp", out.join("env").join("proc_net_udp_end.txt"));
    let _ = fs::copy("/proc/net/snmp", out.join("env").join("proc_net_snmp_end.txt"));

    let sys_tot = report::SysTotals {
        start,
        end: t_end,
        cap_packets: shared.cap_packets.load(Ordering::Relaxed),
        cap_drops: shared.cap_drops.load(Ordering::Relaxed),
        rcvbuf_errors: kernel::read_snmp_udp().get("RcvbufErrors").copied().unwrap_or(0)
            .saturating_sub(snmp_start.get("RcvbufErrors").copied().unwrap_or(0)),
        softnet_dropped: kernel::read_softnet().0.saturating_sub(soft_start.0),
        softnet_squeeze: kernel::read_softnet().1.saturating_sub(soft_start.1),
        nic: ifaces
            .iter()
            .map(|i| {
                let cur = kernel::read_nic(i);
                let d: Vec<(String, u64)> = cur
                    .iter()
                    .map(|(k, v)| (k.clone(), v.saturating_sub(*nic_start[i].get(k).unwrap_or(v))))
                    .collect();
                (i.clone(), d)
            })
            .collect(),
        events: event_id,
        nimble_pid,
    };
    let mut text = report::summary(&shared, &sys_tot, &opts);
    let check = stall::Inputs { dir: &out, stream: None, dns: None, strace: None, nimble_log: None };
    match stall::analyze(&check) {
        Ok(r) => {
            text.push_str("\n==================== ingest stall check (nimdiag analyze) ====================\n");
            text.push_str(&stall::report_text(&r));
        }
        Err(e) => text.push_str(&format!("\ningest stall check failed: {}\n", e)),
    }
    let _ = fs::write(out.join("summary.txt"), &text);
    println!("\n{}", text);
    log.line(&format!("{} finished", fmt_utc(t_end)));

    if opts.tar {
        let parent = out.parent().unwrap_or(Path::new("."));
        let tgz = parent.join(format!("{}.tar.gz", dir_name));
        let st = std::process::Command::new("tar")
            .arg("czf")
            .arg(&tgz)
            .arg("-C")
            .arg(parent)
            .arg(&dir_name)
            .status();
        match st {
            Ok(s) if s.success() => println!("archive: {}", tgz.display()),
            _ => println!("tar failed, results are in {}", out.display()),
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn finish_event(
    p: &Pending,
    shared: &Shared,
    history: &[VecDeque<(u64, Value)>],
    tmon: Option<&threads::ThreadMon>,
    opts: &Opts,
    out: &Path,
    disk_bytes: &mut u64,
    f_events: &mut File,
    log: &mut Log,
) {
    let from = p.t0.saturating_sub(opts.pre * 1_000_000_000);
    let to = p.due.max(p.t0);
    let (name, label) = {
        let g = shared.groups[p.group].lock().unwrap();
        (g.cfg.name.clone(), g.cfg.label())
    };
    let base = format!("event_{:03}_{}", p.id, sanitize(&name));

    // pcap from packet ring(s)
    let mut pcap_info = json!(null);
    if opts.ring_mb > 0 {
        if *disk_bytes / (1024 * 1024) >= opts.max_disk_mb {
            pcap_info = json!({"skipped": "max-disk-mb reached"});
        } else {
            let mut frames: Vec<(u64, Vec<u8>)> = Vec::new();
            let mut oldest = Vec::new();
            let mut truncated = false;
            let gis: Vec<usize> = if opts.event_pcap_all { (0..shared.groups.len()).collect() } else { vec![p.group] };
            for gi in gis {
                let g = shared.groups[gi].lock().unwrap();
                let (mut f, o) = g.ring.snapshot(from, to);
                truncated |= g.ring.truncated && o.map(|o| o > from).unwrap_or(false);
                if let Some(o) = o {
                    oldest.push(json!({"group": g.cfg.label(), "oldest": fmt_utc_ms(o)}));
                }
                frames.append(&mut f);
            }
            frames.sort_by_key(|(t, _)| *t);
            let file = format!("{}.pcap", base);
            match recorder::write_pcap(&out.join("events").join(&file), &frames) {
                Ok(b) => {
                    *disk_bytes += b;
                    pcap_info = json!({"file": file, "frames": frames.len(), "bytes": b,
                        "window_truncated_by_ring_mb": truncated, "ring_oldest": oldest});
                }
                Err(e) => pcap_info = json!({"error": e.to_string()}),
            }
        }
    }

    let hist: Vec<Value> = history[p.group]
        .iter()
        .filter(|(t, _)| *t >= from && *t <= to + 1_000_000_000)
        .map(|(_, v)| v.clone())
        .collect();

    let thr: Vec<Value> = tmon
        .map(|m| {
            m.ring_window(from, to)
                .into_iter()
                .map(|(t, views)| {
                    let v: Vec<&threads::View> = views
                        .iter()
                        .filter(|v| threads::is_interesting(&v.name) || v.cpu_pct >= 1.0 || v.state == 'D' || v.state == 'R')
                        .collect();
                    json!({"t": fmt_utc_ms(t), "threads": v})
                })
                .collect()
        })
        .unwrap_or_default();

    let ev = json!({
        "id": p.id,
        "t": fmt_utc_ms(p.t0),
        "stream": name,
        "group": label,
        "reason": p.reason,
        "window": [fmt_utc_ms(from), fmt_utc_ms(to)],
        "at_trigger": p.snapshot,
        "stream_seconds": hist,
        "pcap": pcap_info,
        "nimble_threads": thr,
    });
    let file = out.join("events").join(format!("{}.json", base));
    let _ = fs::write(&file, serde_json::to_string_pretty(&ev).unwrap_or_default());
    let _ = writeln!(f_events, "{}", json!({"id": p.id, "t": fmt_utc_ms(p.t0), "stream": name, "group": label,
        "reason": p.reason, "file": format!("events/{}.json", base), "pcap": pcap_info}));
    log.line(&format!("{} event #{} written: events/{}.json", fmt_utc(now_ns()), p.id, base));
}

fn print_console(
    log: &mut Log,
    t: u64,
    shared: &Shared,
    con: &mut [(u64, u64, u64, u64, u64, u64)],
    cap_drop_d: u64,
    rcvbuf_err_d: u64,
    pid: Option<i32>,
) {
    log.line(&format!(
        "\n{} | capture drops/s {} | UDP RcvbufErrors/s {} | nimble pid {}",
        fmt_utc(t), cap_drop_d, rcvbuf_err_d, pid.map(|p| p.to_string()).unwrap_or("-".into())
    ));
    log.line(&format!(
        "{:<34} {:<22} {:>8} {:>7} {:>6} {:>7} {:>8} {:>8} {:>5}  {}",
        "STREAM", "GROUP", "Mbps", "pps", "size", "b1ms", "rxqKiB", "drops/s", "cc", "FLAGS"
    ));
    let mut seen: HashSet<usize> = HashSet::new();
    for (gi, c) in con.iter_mut().enumerate() {
        if !seen.insert(gi) {
            continue;
        }
        let g = shared.groups[gi].lock().unwrap();
        let secs = c.5.max(1) as f64;
        let mbps = c.1 as f64 * 8.0 / 1e6 / secs;
        let pps = c.0 as f64 / secs;
        let size = top_sizes(&g.total.sizes, 1).first().map(|x| x.0.to_string()).unwrap_or("-".into());
        let mut flags = Vec::new();
        if c.0 == 0 {
            flags.push("NODATA");
        }
        if g.total.rtp_pkts > 0 {
            flags.push("RTP");
        }
        if c.3 > 0 {
            flags.push("!CC");
        }
        if c.2 > 0 {
            flags.push("!DROP");
        }
        if c.4 >= 1_048_576 {
            flags.push("!RXQ");
        }
        if g.total.sock_seen == false {
            flags.push("NOSOCK");
        }
        let mut nm = g.cfg.name.clone();
        if nm.len() > 34 {
            nm.truncate(33);
            nm.push('~');
        }
        log.line(&format!(
            "{:<34} {:<22} {:>8.3} {:>7.0} {:>6} {:>7} {:>8} {:>8.0} {:>5}  {}",
            nm, g.cfg.label(), mbps, pps, size, g.total.burst_1ms, c.4 / 1024, c.2 as f64 / secs, c.3, flags.join(" ")
        ));
        *c = (0, 0, 0, 0, 0, 0);
    }
}
