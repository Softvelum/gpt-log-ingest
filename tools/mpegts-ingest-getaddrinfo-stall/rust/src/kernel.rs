// Kernel-side counters: /proc/net/udp, /proc/net/snmp, softnet_stat, NIC statistics, socket owners.

use std::collections::HashMap;
use std::net::Ipv4Addr;

#[derive(Clone, Debug)]
pub struct UdpSock {
    pub ip: u32,
    pub port: u16,
    pub rxq: u64,
    pub drops: u64,
    pub inode: u64,
    pub rport: u16,
    pub state: u8,
}

pub fn read_udp() -> Vec<UdpSock> {
    let mut out = Vec::new();
    let text = match std::fs::read_to_string("/proc/net/udp") {
        Ok(t) => t,
        Err(_) => return out,
    };
    for line in text.lines().skip(1) {
        let f: Vec<&str> = line.split_whitespace().collect();
        if f.len() < 13 {
            continue;
        }
        let mut la = f[1].split(':');
        let (ip_hex, port_hex) = match (la.next(), la.next()) {
            (Some(a), Some(b)) => (a, b),
            _ => continue,
        };
        let raw = match u32::from_str_radix(ip_hex, 16) {
            Ok(v) => v,
            Err(_) => continue,
        };
        // /proc/net/udp prints the address as a native-endian u32 (little-endian on x86)
        let ip = u32::from(Ipv4Addr::from(raw.to_le_bytes()));
        let port = u16::from_str_radix(port_hex, 16).unwrap_or(0);
        let rxq = f[4].split(':').nth(1).and_then(|s| u64::from_str_radix(s, 16).ok()).unwrap_or(0);
        let inode = f[9].parse().unwrap_or(0);
        let drops = f[12].parse().unwrap_or(0);
        let rport = f[2].split(':').nth(1).and_then(|s| u16::from_str_radix(s, 16).ok()).unwrap_or(0);
        let state = u8::from_str_radix(f[3], 16).unwrap_or(0);
        out.push(UdpSock { ip, port, rxq, drops, inode, rport, state });
    }
    out
}

/// Key/value pairs from the "Udp:" section of /proc/net/snmp.
pub fn read_snmp_udp() -> HashMap<String, u64> {
    let mut m = HashMap::new();
    let text = std::fs::read_to_string("/proc/net/snmp").unwrap_or_default();
    let lines: Vec<&str> = text.lines().filter(|l| l.starts_with("Udp:")).collect();
    if lines.len() >= 2 {
        let keys: Vec<&str> = lines[0].split_whitespace().skip(1).collect();
        let vals: Vec<&str> = lines[1].split_whitespace().skip(1).collect();
        for (k, v) in keys.iter().zip(vals.iter()) {
            m.insert(k.to_string(), v.parse().unwrap_or(0));
        }
    }
    m
}

/// Sum over CPUs of (dropped, time_squeeze) from /proc/net/softnet_stat.
pub fn read_softnet() -> (u64, u64) {
    let text = std::fs::read_to_string("/proc/net/softnet_stat").unwrap_or_default();
    let mut d = 0;
    let mut s = 0;
    for line in text.lines() {
        let f: Vec<&str> = line.split_whitespace().collect();
        if f.len() >= 3 {
            d += u64::from_str_radix(f[1], 16).unwrap_or(0);
            s += u64::from_str_radix(f[2], 16).unwrap_or(0);
        }
    }
    (d, s)
}

pub const NIC_KEYS: &[&str] = &[
    "rx_packets",
    "rx_bytes",
    "rx_dropped",
    "rx_errors",
    "rx_missed_errors",
    "rx_fifo_errors",
    "rx_over_errors",
    "rx_crc_errors",
    "multicast",
];

pub fn read_nic(iface: &str) -> HashMap<String, u64> {
    let mut m = HashMap::new();
    for k in NIC_KEYS {
        let p = format!("/sys/class/net/{}/statistics/{}", iface, k);
        if let Ok(v) = std::fs::read_to_string(&p) {
            m.insert(k.to_string(), v.trim().parse().unwrap_or(0));
        }
    }
    m
}

/// Socket inodes held open by one process.
fn socket_inodes(pid: i32) -> Vec<u64> {
    let mut out = Vec::new();
    if let Ok(rd) = std::fs::read_dir(format!("/proc/{}/fd", pid)) {
        for fd in rd.flatten() {
            if let Ok(link) = std::fs::read_link(fd.path()) {
                let s = link.to_string_lossy();
                if let Some(rest) = s.strip_prefix("socket:[") {
                    if let Ok(ino) = rest.trim_end_matches(']').parse::<u64>() {
                        out.push(ino);
                    }
                }
            }
        }
    }
    out
}

/// Connected UDP sockets to port 53 (glibc resolver queries in flight) held by the
/// Nimble process; all such sockets on the host when the process is unknown.
pub fn dns_sockets(udp: &[UdpSock], pid: Option<i32>) -> usize {
    let dns: Vec<u64> = udp.iter().filter(|u| u.rport == 53 && u.state == 1).map(|u| u.inode).collect();
    if dns.is_empty() {
        return 0;
    }
    match pid {
        Some(p) => {
            let own = socket_inodes(p);
            dns.iter().filter(|i| own.contains(i)).count()
        }
        None => dns.len(),
    }
}

/// Map socket inode -> (pid, comm). The preferred (Nimble) process is checked first;
/// other processes in /proc are scanned only for inodes it does not hold.
pub fn inode_owners(wanted: &[u64], prefer: Option<i32>) -> HashMap<u64, (i32, String)> {
    let mut out = HashMap::new();
    if wanted.is_empty() {
        return out;
    }
    if let Some(p) = prefer {
        let comm = std::fs::read_to_string(format!("/proc/{}/comm", p)).map(|x| x.trim().to_string()).unwrap_or_default();
        for ino in socket_inodes(p) {
            if wanted.contains(&ino) {
                out.insert(ino, (p, comm.clone()));
            }
        }
        if wanted.iter().all(|i| out.contains_key(i)) {
            return out;
        }
    }
    let rd = match std::fs::read_dir("/proc") {
        Ok(r) => r,
        Err(_) => return out,
    };
    for e in rd.flatten() {
        let name = e.file_name();
        let pid: i32 = match name.to_str().and_then(|s| s.parse().ok()) {
            Some(p) => p,
            None => continue,
        };
        if Some(pid) == prefer {
            continue;
        }
        let fds = match std::fs::read_dir(format!("/proc/{}/fd", pid)) {
            Ok(r) => r,
            Err(_) => continue,
        };
        let mut comm: Option<String> = None;
        for fd in fds.flatten() {
            if let Ok(link) = std::fs::read_link(fd.path()) {
                let s = link.to_string_lossy();
                if let Some(rest) = s.strip_prefix("socket:[") {
                    if let Ok(ino) = rest.trim_end_matches(']').parse::<u64>() {
                        if wanted.contains(&ino) {
                            let c = comm.get_or_insert_with(|| {
                                std::fs::read_to_string(format!("/proc/{}/comm", pid))
                                    .map(|x| x.trim().to_string())
                                    .unwrap_or_default()
                            });
                            out.insert(ino, (pid, c.clone()));
                        }
                    }
                }
            }
        }
    }
    out
}
