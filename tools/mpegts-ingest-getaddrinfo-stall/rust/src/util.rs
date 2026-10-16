use std::net::Ipv4Addr;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn now_ns() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(0)
}

// Days since 1970-01-01 -> (year, month, day). Algorithm by Howard Hinnant.
fn civil_from_days(days: i64) -> (i64, u32, u32) {
    let z = days + 719_468;
    let era = (if z >= 0 { z } else { z - 146_096 }) / 146_097;
    let doe = (z - era * 146_097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = (if mp < 10 { mp + 3 } else { mp - 9 }) as u32;
    (if m <= 2 { y + 1 } else { y }, m, d)
}

fn split(ns: u64) -> (i64, u32, u32, u64, u64, u64, u64) {
    let secs = ns / 1_000_000_000;
    let (y, mo, d) = civil_from_days((secs / 86_400) as i64);
    let rem = secs % 86_400;
    (y, mo, d, rem / 3600, (rem % 3600) / 60, rem % 60, (ns % 1_000_000_000) / 1_000_000)
}

/// "%Y-%m-%d %H:%M:%S" in UTC
pub fn fmt_utc(ns: u64) -> String {
    let (y, mo, d, h, mi, s, _) = split(ns);
    format!("{:04}-{:02}-{:02} {:02}:{:02}:{:02}", y, mo, d, h, mi, s)
}

/// "%Y-%m-%d %H:%M:%S.mmm" in UTC
pub fn fmt_utc_ms(ns: u64) -> String {
    let (y, mo, d, h, mi, s, ms) = split(ns);
    format!("{:04}-{:02}-{:02} {:02}:{:02}:{:02}.{:03}", y, mo, d, h, mi, s, ms)
}

/// "%Y%m%d_%H%M%S" in UTC, for file names
pub fn fmt_compact(ns: u64) -> String {
    let (y, mo, d, h, mi, s, _) = split(ns);
    format!("{:04}{:02}{:02}_{:02}{:02}{:02}", y, mo, d, h, mi, s)
}

pub fn ip_from_key(k: u32) -> Ipv4Addr {
    Ipv4Addr::from(k)
}

pub fn read_to_string(path: &str) -> Option<String> {
    std::fs::read_to_string(path).ok()
}

pub fn hostname() -> String {
    read_to_string("/proc/sys/kernel/hostname")
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "host".to_string())
}

pub fn sanitize(name: &str) -> String {
    name.chars()
        .map(|c| if c.is_ascii_alphanumeric() || c == '.' || c == '-' { c } else { '_' })
        .collect()
}
