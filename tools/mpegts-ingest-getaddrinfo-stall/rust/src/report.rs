// Final summary with automatic findings.

use crate::capture::Shared;
use crate::util::*;
use crate::Opts;

pub struct SysTotals {
    pub start: u64,
    pub end: u64,
    pub cap_packets: u64,
    pub cap_drops: u64,
    pub rcvbuf_errors: u64,
    pub softnet_dropped: u64,
    pub softnet_squeeze: u64,
    pub nic: Vec<(String, Vec<(String, u64)>)>,
    pub events: u64,
    pub nimble_pid: Option<i32>,
}

pub fn summary(shared: &Shared, sys: &SysTotals, opts: &Opts) -> String {
    let mut s = String::new();
    let dur = (sys.end.saturating_sub(sys.start)) as f64 / 1e9;
    s.push_str("==================== nimdiag summary ====================\n");
    s.push_str(&format!("host: {}  run: {} .. {} UTC ({:.0} s)\n", hostname(), fmt_utc(sys.start), fmt_utc(sys.end), dur));
    s.push_str(&format!("nimble pid: {}  config: {}  events: {}\n\n",
        sys.nimble_pid.map(|p| p.to_string()).unwrap_or("not found".into()), opts.config, sys.events));

    let mut findings: Vec<String> = Vec::new();
    let unreliable = sys.cap_drops > 0;

    for g in &shared.groups {
        let g = g.lock().unwrap();
        let t = &g.total;
        let secs = t.seconds.max(1) as f64;
        let avg = t.bytes as f64 * 8.0 / 1e6 / secs;
        let pps = t.pkts as f64 / secs;
        let mut sizes: Vec<(u32, u64)> = t.sizes.iter().map(|(k, v)| (*k, *v)).collect();
        sizes.sort_by(|a, b| b.1.cmp(&a.1));
        let size_str = sizes
            .iter()
            .take(4)
            .map(|(k, v)| format!("{}B x{:.1}%", k, *v as f64 * 100.0 / t.pkts.max(1) as f64))
            .collect::<Vec<_>>()
            .join(", ");
        let pcr_avg = if t.pcr_br_n > 0 { t.pcr_br_sum / t.pcr_br_n as f64 / 1e6 } else { 0.0 };
        let null_pct = if t.ts_pkts > 0 { t.null_pkts as f64 * 100.0 / t.ts_pkts as f64 } else { 0.0 };
        s.push_str(&format!("--- {}  [{}]\n", g.cfg.name, g.cfg.label()));
        s.push_str(&format!(
            "  wire: avg {:.3} Mbps (min {:.3} / max {:.3}), {:.0} pps, data {}/{} s, sources: {}\n",
            avg,
            if t.min_mbps == f64::MAX { 0.0 } else { t.min_mbps },
            t.max_mbps,
            pps,
            t.seconds_with_data,
            t.seconds,
            t.srcs.iter().map(|x| ip_from_key(*x).to_string()).collect::<Vec<_>>().join(",")
        ));
        s.push_str(&format!("  datagrams: {}\n", if size_str.is_empty() { "-".into() } else { size_str }));
        s.push_str(&format!(
            "  timing: max burst {} pkt/1ms, {} pkt/10ms, max gap {:.1} ms, PCR jitter max {:.2} ms\n",
            t.burst_1ms, t.burst_10ms, t.max_gap_ns as f64 / 1e6, t.pcr_jitter_max_ns as f64 / 1e6
        ));
        s.push_str(&format!(
            "  TS: pkts {}, PIDs {}, CC err {}, CC dup {}, sync err {}, TEI {}, scrambled {}, null {:.1}%, PCR bitrate {:.3} Mbps, PCR disc {}\n",
            t.ts_pkts, t.pids.len(), t.cc_err, t.cc_dup, t.sync_err, t.tei, t.scrambled, null_pct, pcr_avg, t.pcr_disc
        ));
        if t.rtp_pkts > 0 || t.frags > 0 || t.non188 > 0 {
            s.push_str(&format!("  encap: RTP pkts {}, RTP seq err {}, IP frags {}, non-188-multiple {}\n",
                t.rtp_pkts, t.rtp_seq_err, t.frags, t.non188));
        }
        s.push_str(&format!(
            "  nimble socket: {}, max rx_queue {} B, drops during run {}, seconds with drops {}, events {}\n\n",
            if t.sock_seen { "present" } else { "NOT FOUND" },
            t.rxq_max,
            t.drops_total,
            t.seconds_with_drops,
            t.events
        ));

        // findings
        let n = format!("{} [{}]", g.cfg.name, g.cfg.label());
        if t.pkts == 0 {
            findings.push(format!("{}: no traffic on the wire at all (source down, wrong interface, or not joined upstream).", n));
            continue;
        }
        if !t.sock_seen {
            findings.push(format!("{}: traffic present but no Nimble UDP socket bound to this group (stream inactive in Nimble?).", n));
        }
        // a few CC errors on the wire do not explain many socket drops: compare the two
        let wire_errs = t.cc_err + t.rtp_seq_err;
        let wire_minor = t.sync_err == 0 && wire_errs * 100 <= t.drops_total;
        if t.drops_total > 0 {
            if wire_minor && !unreliable {
                findings.push(format!(
                    "{}: kernel dropped {} datagrams at Nimble's socket while the wire stream was clean{} -> Nimble is not reading this socket fast enough (application side).",
                    n, t.drops_total,
                    if wire_errs > 0 { format!(" (only {} CC/RTP errors on the wire)", wire_errs) } else { String::new() }));
            } else {
                findings.push(format!("{}: {} drops at Nimble's socket; the wire also shows comparable errors, see below.", n, t.drops_total));
            }
        } else if t.rxq_max >= 1_048_576 {
            findings.push(format!("{}: Nimble socket rx_queue reached {} B (no drops yet) -> reader falls behind.", n, t.rxq_max));
        }
        if wire_errs > 0 && !(t.drops_total > 0 && wire_minor) {
            findings.push(format!(
                "{}: loss/reorder visible on the wire before Nimble (CC err {}, RTP seq err {}) -> upstream source/network problem{}.",
                n, t.cc_err, t.rtp_seq_err, if unreliable { " (or nimdiag capture drops, see global)" } else { "" }));
        }
        if t.sync_err > 0 {
            findings.push(format!("{}: {} TS packets without 0x47 sync -> payload is not plain/aligned MPEG-TS.", n, t.sync_err));
        }
        if t.rtp_pkts > 0 {
            findings.push(format!("{}: RTP encapsulation detected ({} pkts); make sure Nimble expects RTP for this input.", n, t.rtp_pkts));
        }
        if let Some((sz, cnt)) = sizes.first() {
            let share = *cnt as f64 / t.pkts as f64;
            if *sz != 1316 && *sz != 1328 {
                findings.push(format!("{}: dominant datagram size is {} B (not 1316/1328).", n, sz));
            }
            if share < 0.95 && sizes.len() > 1 {
                findings.push(format!("{}: variable datagram sizes ({} distinct, top {:.0}%).", n, sizes.len(), share * 100.0));
            }
        }
        if t.non188 > 0 {
            findings.push(format!("{}: {} datagrams are not a multiple of 188 B.", n, t.non188));
        }
        let avg_per_ms = pps / 1000.0;
        if t.burst_1ms as f64 > (avg_per_ms * 10.0).max(20.0) {
            findings.push(format!(
                "{}: bursty delivery, up to {} packets within 1 ms (average {:.1}/ms).",
                n, t.burst_1ms, avg_per_ms));
        }
        if t.max_gap_ns > 500_000_000 && t.seconds_with_data == t.seconds {
            findings.push(format!("{}: gaps up to {:.0} ms between packets.", n, t.max_gap_ns as f64 / 1e6));
        }
        if t.pcr_jitter_max_ns > 50_000_000 {
            findings.push(format!("{}: high PCR arrival jitter, up to {:.1} ms.", n, t.pcr_jitter_max_ns as f64 / 1e6));
        }
        if t.tei > 0 {
            findings.push(format!("{}: {} TS packets with transport error indicator.", n, t.tei));
        }
        if t.frags > 0 {
            findings.push(format!("{}: {} IP fragments received.", n, t.frags));
        }
        if t.srcs.len() > 1 {
            findings.push(format!("{}: {} different source IPs send to this group.", n, t.srcs.len()));
        }
        if t.ts_pkts > 0 && t.scrambled as f64 / t.ts_pkts as f64 > 0.01 {
            findings.push(format!("{}: scrambled TS packets present ({}).", n, t.scrambled));
        }
    }

    s.push_str("--- global\n");
    s.push_str(&format!("  nimdiag capture: {} packets passed filter (on loopback includes locally sent copies), {} dropped by capture socket\n", sys.cap_packets, sys.cap_drops));
    s.push_str(&format!("  UDP RcvbufErrors (all sockets): {}\n", sys.rcvbuf_errors));
    s.push_str(&format!("  softnet: dropped {}, time_squeeze {}\n", sys.softnet_dropped, sys.softnet_squeeze));
    for (i, d) in &sys.nic {
        let mut d = d.clone();
        d.sort();
        s.push_str(&format!("  nic {}: {}\n", i,
            d.iter().map(|(k, v)| format!("{}={}", k, v)).collect::<Vec<_>>().join(" ")));
        let bad: u64 = d
            .iter()
            .filter(|(k, _)| k != "rx_packets" && k != "rx_bytes" && k != "multicast")
            .map(|(_, v)| *v)
            .sum();
        if bad > 0 {
            findings.push(format!("host: NIC {} reports {} rx drop/error counts during run -> loss before sockets.", i, bad));
        }
    }
    if let Ok(o) = shared.other.lock() {
        if !o.is_empty() {
            let mut v: Vec<(&(u32, u16), &(u64, u64))> = o.iter().collect();
            v.sort_by(|a, b| b.1 .1.cmp(&a.1 .1));
            s.push_str("  other multicast on capture interfaces (not in config), top 15 by bytes:\n");
            for ((ip, port), (p, b)) in v.iter().take(15) {
                s.push_str(&format!("    {}:{}  pkts {}  avg {:.3} Mbps\n", ip_from_key(*ip), port, p, *b as f64 * 8.0 / 1e6 / dur.max(1.0)));
            }
        }
    }
    if sys.cap_drops > 0 {
        findings.push(format!("nimdiag: capture socket dropped {} packets; wire statistics are incomplete (increase --rcvbuf-mb).", sys.cap_drops));
    }
    if sys.softnet_dropped > 0 {
        findings.push(format!("host: softnet backlog dropped {} packets (netdev_max_backlog / CPU) -> loss before sockets.", sys.softnet_dropped));
    }

    s.push_str("\n==================== findings ====================\n");
    if findings.is_empty() {
        s.push_str("  nothing abnormal detected during this run.\n");
    } else {
        for f in findings {
            s.push_str(&format!("  * {}\n", f));
        }
    }
    s
}
