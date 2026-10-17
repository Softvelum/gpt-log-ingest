// Per-stream analysis: rate, datagram sizes, bursts, RTP, MPEG-TS (CC, PCR), packet ring.

use crate::config::Stream;
use std::collections::{BTreeMap, BTreeSet, HashMap, VecDeque};

const PCR_WRAP: u64 = (1u64 << 33) * 300;
const PCR_HZ: u64 = 27_000_000;

#[derive(Default, Clone)]
pub struct SecAcc {
    pub pkts: u64,
    pub bytes: u64,
    pub sizes: BTreeMap<u32, u64>,
    pub burst_1ms: u32,
    pub burst_10ms: u32,
    pub max_gap_ns: u64,
    pub srcs: BTreeSet<u32>,
    pub ttl_min: u8,
    pub ttl_max: u8,
    pub frags: u64,
    pub rtp_pkts: u64,
    pub rtp_seq_err: u64,
    pub ts_pkts: u64,
    pub non188: u64,
    pub sync_err: u64,
    pub cc_err: u64,
    pub cc_dup: u64,
    pub tei: u64,
    pub scrambled: u64,
    pub null_pkts: u64,
    pub pcr_count: u64,
    pub pcr_disc: u64,
    pub pcr_jitter_max_ns: u64,
    pub pcr_br_sum: f64,
    pub pcr_br_n: u64,
    pub pids: BTreeSet<u16>,
}

#[derive(Default, Clone)]
pub struct Totals {
    pub seconds: u64,
    pub seconds_with_data: u64,
    pub pkts: u64,
    pub bytes: u64,
    pub sizes: BTreeMap<u32, u64>,
    pub min_mbps: f64,
    pub max_mbps: f64,
    pub burst_1ms: u32,
    pub burst_10ms: u32,
    pub max_gap_ns: u64,
    pub srcs: BTreeSet<u32>,
    pub frags: u64,
    pub rtp_pkts: u64,
    pub rtp_seq_err: u64,
    pub ts_pkts: u64,
    pub non188: u64,
    pub sync_err: u64,
    pub cc_err: u64,
    pub cc_dup: u64,
    pub tei: u64,
    pub scrambled: u64,
    pub null_pkts: u64,
    pub pcr_disc: u64,
    pub pcr_jitter_max_ns: u64,
    pub pcr_br_sum: f64,
    pub pcr_br_n: u64,
    pub pids: BTreeSet<u16>,
    // kernel-side, filled by main loop
    pub sock_seen: bool,
    pub rxq_max: u64,
    pub drops_total: u64,
    pub seconds_with_drops: u64,
    pub events: u64,
}

pub struct Ring {
    q: VecDeque<(u64, Vec<u8>)>,
    bytes: usize,
    cap_bytes: usize,
    keep_ns: u64,
    pub truncated: bool,
}

impl Ring {
    pub fn new(cap_bytes: usize, keep_ns: u64) -> Self {
        Ring { q: VecDeque::new(), bytes: 0, cap_bytes, keep_ns, truncated: false }
    }
    fn push(&mut self, ts: u64, frame: &[u8]) {
        if self.cap_bytes == 0 {
            return;
        }
        self.q.push_back((ts, frame.to_vec()));
        self.bytes += frame.len();
        while let Some((t0, f)) = self.q.front() {
            let too_old = ts.saturating_sub(*t0) > self.keep_ns;
            let too_big = self.bytes > self.cap_bytes;
            if !too_old && !too_big {
                break;
            }
            if too_big && !too_old {
                self.truncated = true;
            }
            self.bytes -= f.len();
            self.q.pop_front();
        }
    }
    pub fn snapshot(&self, from: u64, to: u64) -> (Vec<(u64, Vec<u8>)>, Option<u64>) {
        let oldest = self.q.front().map(|(t, _)| *t);
        let v = self
            .q
            .iter()
            .filter(|(t, _)| *t >= from && *t <= to)
            .cloned()
            .collect();
        (v, oldest)
    }
}

struct PcrState {
    last_pcr: u64,
    last_ts: u64,
    last_count: u64,
}

pub struct Group {
    pub cfg: Stream,
    pub sec: SecAcc,
    pub total: Totals,
    pub ring: Ring,
    last_ts_ns: u64,
    bin1: (u64, u32),
    bin10: (u64, u32),
    cc: HashMap<u16, u8>,
    pcr: HashMap<u16, PcrState>,
    ts_counter: u64,
    rtp_last_seq: Option<u16>,
}

impl Group {
    pub fn new(cfg: Stream, ring_bytes: usize, keep_ns: u64) -> Self {
        let total = Totals { min_mbps: f64::MAX, ..Default::default() };
        Group {
            cfg,
            sec: SecAcc::default(),
            total,
            ring: Ring::new(ring_bytes, keep_ns),
            last_ts_ns: 0,
            bin1: (0, 0),
            bin10: (0, 0),
            cc: HashMap::new(),
            pcr: HashMap::new(),
            ts_counter: 0,
            rtp_last_seq: None,
        }
    }

    pub fn on_packet(&mut self, ts: u64, frame: &[u8], payload: &[u8], src: u32, ttl: u8, frag: bool) {
        self.ring.push(ts, frame);
        let a = &mut self.sec;
        if a.pkts == 0 {
            a.ttl_min = ttl;
            a.ttl_max = ttl;
        } else {
            a.ttl_min = a.ttl_min.min(ttl);
            a.ttl_max = a.ttl_max.max(ttl);
        }
        a.pkts += 1;
        a.bytes += payload.len() as u64;
        *a.sizes.entry(payload.len() as u32).or_insert(0) += 1;
        if a.srcs.len() < 16 {
            a.srcs.insert(src);
        }
        // bursts: max packets within one 1 ms / 10 ms bin
        let b1 = ts / 1_000_000;
        if self.bin1.0 == b1 { self.bin1.1 += 1 } else { self.bin1 = (b1, 1) }
        a.burst_1ms = a.burst_1ms.max(self.bin1.1);
        let b10 = ts / 10_000_000;
        if self.bin10.0 == b10 { self.bin10.1 += 1 } else { self.bin10 = (b10, 1) }
        a.burst_10ms = a.burst_10ms.max(self.bin10.1);
        if self.last_ts_ns != 0 && ts > self.last_ts_ns {
            a.max_gap_ns = a.max_gap_ns.max(ts - self.last_ts_ns);
        }
        self.last_ts_ns = ts;
        if frag {
            a.frags += 1;
            return;
        }

        // RTP detection: version 2 and not starting with TS sync byte
        let mut off = 0usize;
        if payload.len() >= 12 && payload[0] & 0xC0 == 0x80 && payload[0] != 0x47 {
            a.rtp_pkts += 1;
            let csrc = (payload[0] & 0x0F) as usize;
            off = 12 + csrc * 4;
            if payload[0] & 0x10 != 0 && payload.len() >= off + 4 {
                let ext_len = u16::from_be_bytes([payload[off + 2], payload[off + 3]]) as usize;
                off += 4 + ext_len * 4;
            }
            let seq = u16::from_be_bytes([payload[2], payload[3]]);
            if let Some(last) = self.rtp_last_seq {
                if seq != last.wrapping_add(1) {
                    a.rtp_seq_err += 1;
                }
            }
            self.rtp_last_seq = Some(seq);
            if off > payload.len() {
                return;
            }
        }
        let ts_data = &payload[off..];
        if ts_data.len() % 188 != 0 {
            self.sec.non188 += 1;
        }
        let mut i = 0;
        while i + 188 <= ts_data.len() {
            let p = &ts_data[i..i + 188];
            i += 188;
            self.ts_packet(ts, p);
        }
    }

    fn ts_packet(&mut self, ts: u64, p: &[u8]) {
        self.ts_counter += 1;
        let a = &mut self.sec;
        a.ts_pkts += 1;
        if p[0] != 0x47 {
            a.sync_err += 1;
            return;
        }
        if p[1] & 0x80 != 0 {
            a.tei += 1;
        }
        let pid = (((p[1] & 0x1F) as u16) << 8) | p[2] as u16;
        if pid == 0x1FFF {
            a.null_pkts += 1;
            return;
        }
        if a.pids.len() < 512 {
            a.pids.insert(pid);
        }
        if p[3] >> 6 != 0 {
            a.scrambled += 1;
        }
        let afc = (p[3] >> 4) & 3;
        let cc = p[3] & 0x0F;
        let mut disc = false;
        if afc & 2 != 0 {
            let al = p[4] as usize;
            if al > 0 && al <= 183 {
                let flags = p[5];
                disc = flags & 0x80 != 0;
                if flags & 0x10 != 0 && al >= 7 {
                    let base = ((p[6] as u64) << 25)
                        | ((p[7] as u64) << 17)
                        | ((p[8] as u64) << 9)
                        | ((p[9] as u64) << 1)
                        | ((p[10] as u64) >> 7);
                    let ext = (((p[10] & 1) as u64) << 8) | p[11] as u64;
                    self.on_pcr(pid, base * 300 + ext, ts, disc);
                }
            }
        }
        if afc & 1 != 0 {
            let a = &mut self.sec;
            if let Some(&last) = self.cc.get(&pid) {
                if cc == last {
                    a.cc_dup += 1;
                } else if cc != (last + 1) & 0x0F && !disc {
                    a.cc_err += 1;
                }
            }
            self.cc.insert(pid, cc);
        }
    }

    fn on_pcr(&mut self, pid: u16, pcr: u64, ts: u64, disc: bool) {
        let counter = self.ts_counter;
        let a = &mut self.sec;
        a.pcr_count += 1;
        if let Some(st) = self.pcr.get(&pid) {
            let dp = (pcr + PCR_WRAP - st.last_pcr) % PCR_WRAP;
            if !disc && dp > 0 && dp < 2 * PCR_HZ && ts > st.last_ts {
                let dp_ns = dp * 1000 / 27;
                let da_ns = ts - st.last_ts;
                let jit = (da_ns as i64 - dp_ns as i64).unsigned_abs();
                a.pcr_jitter_max_ns = a.pcr_jitter_max_ns.max(jit);
                let bits = (counter - st.last_count) as f64 * 188.0 * 8.0;
                a.pcr_br_sum += bits * PCR_HZ as f64 / dp as f64;
                a.pcr_br_n += 1;
            } else {
                a.pcr_disc += 1;
            }
        }
        self.pcr.insert(pid, PcrState { last_pcr: pcr, last_ts: ts, last_count: counter });
    }

    /// Close the current one-second window, fold it into totals and return it.
    pub fn take_second(&mut self) -> SecAcc {
        let s = std::mem::take(&mut self.sec);
        let t = &mut self.total;
        t.seconds += 1;
        let mbps = s.bytes as f64 * 8.0 / 1e6;
        if s.pkts > 0 {
            t.seconds_with_data += 1;
        }
        t.min_mbps = t.min_mbps.min(mbps);
        t.max_mbps = t.max_mbps.max(mbps);
        t.pkts += s.pkts;
        t.bytes += s.bytes;
        for (k, v) in &s.sizes {
            *t.sizes.entry(*k).or_insert(0) += v;
        }
        t.burst_1ms = t.burst_1ms.max(s.burst_1ms);
        t.burst_10ms = t.burst_10ms.max(s.burst_10ms);
        t.max_gap_ns = t.max_gap_ns.max(s.max_gap_ns);
        for x in &s.srcs {
            if t.srcs.len() < 16 {
                t.srcs.insert(*x);
            }
        }
        t.frags += s.frags;
        t.rtp_pkts += s.rtp_pkts;
        t.rtp_seq_err += s.rtp_seq_err;
        t.ts_pkts += s.ts_pkts;
        t.non188 += s.non188;
        t.sync_err += s.sync_err;
        t.cc_err += s.cc_err;
        t.cc_dup += s.cc_dup;
        t.tei += s.tei;
        t.scrambled += s.scrambled;
        t.null_pkts += s.null_pkts;
        t.pcr_disc += s.pcr_disc;
        t.pcr_jitter_max_ns = t.pcr_jitter_max_ns.max(s.pcr_jitter_max_ns);
        t.pcr_br_sum += s.pcr_br_sum;
        t.pcr_br_n += s.pcr_br_n;
        for p in &s.pids {
            if t.pids.len() < 512 {
                t.pids.insert(*p);
            }
        }
        s
    }
}
