// Offline analysis of a nimdiag result directory for the blocked-resolver ingest stall.
// Mirrors python/ingest_stall.py: same checks, same JSON report (see ../README.md).

use serde_json::{json, Map, Value};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs;
use std::net::Ipv4Addr;
use std::path::Path;

pub const VERSION: u64 = 1;
const POLL: &[&str] = &["7", "271"];
const FULL_RXQ: u64 = 1_048_576;
const RCODES: &[&str] = &["NoError", "FormErr", "ServFail", "NXDomain", "NotImp", "Refused"];

// ------------------------------------------------------------------ helpers

fn r2(x: f64) -> f64 {
    format!("{:.2}", x).parse().unwrap_or(0.0)
}

fn median_f(v: &[f64]) -> f64 {
    let mut v = v.to_vec();
    v.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    if v.is_empty() { 0.0 } else { v[v.len() / 2] }
}

fn median_u(v: &[u64]) -> u64 {
    let mut v = v.to_vec();
    v.sort();
    if v.is_empty() { 0 } else { v[v.len() / 2] }
}


fn read_lines(path: &Path) -> Result<Vec<String>, String> {
    let b = fs::read(path).map_err(|e| format!("cannot read {}: {}", path.display(), e))?;
    Ok(String::from_utf8_lossy(&b).lines().map(|s| s.to_string()).collect())
}

fn read_jsonl(path: &Path) -> Vec<Value> {
    read_lines(path)
        .unwrap_or_default()
        .iter()
        .filter_map(|l| {
            let l = l.trim();
            if l.is_empty() { None } else { serde_json::from_str(l).ok() }
        })
        .collect()
}

fn u(v: &Value, k: &str) -> u64 {
    v.get(k).and_then(|x| x.as_u64().or_else(|| x.as_f64().map(|f| f as u64))).unwrap_or(0)
}

fn f(v: &Value, k: &str) -> f64 {
    v.get(k).and_then(|x| x.as_f64()).unwrap_or(0.0)
}

fn s<'a>(v: &'a Value, k: &str) -> &'a str {
    v.get(k).and_then(|x| x.as_str()).unwrap_or("")
}

/// syscall fields are strings in nimdiag output; accept numbers too.
fn sval(v: &Value, k: &str) -> String {
    match v.get(k) {
        Some(Value::String(x)) => x.clone(),
        Some(Value::Number(n)) => n.to_string(),
        _ => String::new(),
    }
}

fn ms_of_day(t: &str) -> u64 {
    let p = t.get(11..).unwrap_or("");
    let mut it = p.splitn(3, ':');
    let hh: u64 = it.next().and_then(|x| x.parse().ok()).unwrap_or(0);
    let mm: u64 = it.next().and_then(|x| x.parse().ok()).unwrap_or(0);
    let rest = it.next().unwrap_or("0");
    let (ss, ms) = match rest.find('.') {
        Some(i) => (&rest[..i], &rest[i + 1..]),
        None => (rest, ""),
    };
    let ss: u64 = ss.parse().unwrap_or(0);
    let ms: u64 = format!("{}000", ms)[..3].parse().unwrap_or(0);
    ((hh * 60 + mm) * 60 + ss) * 1000 + ms
}

fn sec_key(t: &str) -> String {
    t.chars().take(19).collect()
}

// ------------------------------------------------------------------ /proc/net/udp

pub struct ProcSock {
    pub local: String,
    pub remote: String,
    pub remote_ip: String,
    pub remote_port: u16,
    pub state: String,
    pub rxq: u64,
    pub inode: u64,
    pub drops: u64,
}

fn hex_ip(h: &str) -> Option<String> {
    let raw = u32::from_str_radix(h, 16).ok()?;
    Some(Ipv4Addr::from(raw.to_le_bytes()).to_string())
}

pub fn parse_proc_udp(lines: &[String]) -> Vec<ProcSock> {
    let mut out = Vec::new();
    for line in lines {
        let fl: Vec<&str> = line.split_whitespace().collect();
        if fl.len() < 13 || fl[0] == "sl" || !fl[1].contains(':') || !fl[2].contains(':') {
            continue;
        }
        let parse = || -> Option<ProcSock> {
            let (la, lp) = fl[1].split_once(':')?;
            let (ra, rp) = fl[2].split_once(':')?;
            let lip = hex_ip(la)?;
            let rip = hex_ip(ra)?;
            let lport = u16::from_str_radix(lp, 16).ok()?;
            let rport = u16::from_str_radix(rp, 16).ok()?;
            Some(ProcSock {
                local: format!("{}:{}", lip, lport),
                remote: format!("{}:{}", rip, rport),
                remote_ip: rip,
                remote_port: rport,
                state: fl[3].to_string(),
                rxq: u64::from_str_radix(fl[4].split(':').nth(1)?, 16).ok()?,
                inode: fl[9].parse().ok()?,
                drops: fl[12].parse().ok()?,
            })
        };
        if let Some(p) = parse() {
            out.push(p);
        }
    }
    out
}

fn dns_sockets(socks: &[ProcSock]) -> Value {
    let mut c: BTreeMap<String, u64> = BTreeMap::new();
    for x in socks {
        if x.remote_port == 53 && x.state == "01" {
            *c.entry(x.remote_ip.clone()).or_default() += 1;
        }
    }
    Value::Array(c.into_iter().map(|(k, n)| json!({"server": k, "count": n})).collect())
}

pub fn udp_report(path: &Path) -> Result<(Value, String), String> {
    let socks = parse_proc_udp(&read_lines(path)?);
    let limit = socks.iter().map(|x| x.rxq).max().unwrap_or(0);
    let is_full = |x: &ProcSock| x.rxq >= FULL_RXQ && x.rxq * 10 >= limit * 9;
    let dns = dns_sockets(&socks);
    let report = json!({
        "tool": "mpegts-ingest-getaddrinfo-stall", "version": VERSION, "kind": "udp",
        "sockets": socks.len(),
        "full_queue": socks.iter().filter(|x| is_full(x)).map(|x| x.local.clone()).collect::<Vec<_>>(),
        "with_drops": socks.iter().filter(|x| x.drops > 0)
            .map(|x| json!({"local": x.local, "drops": x.drops})).collect::<Vec<_>>(),
        "dns_sockets": dns,
    });
    let mut t = format!("{:<22} {:<22} {:>3} {:>10} {:>12} {:>10}\n", "local", "remote", "st", "rx_queue", "drops", "inode");
    for x in &socks {
        let mark = if is_full(x) {
            "  <- queue at limit"
        } else if x.remote_port == 53 && x.state == "01" {
            "  <- DNS query in flight"
        } else {
            ""
        };
        t.push_str(&format!("{:<22} {:<22} {:>3} {:>10} {:>12} {:>10}{}\n",
            x.local, x.remote, x.state, x.rxq, x.drops, x.inode, mark));
    }
    t.push('\n');
    t.push_str(&format!("sockets with queue at the limit (>= 1 MiB): {}\n", socks.iter().filter(|x| is_full(x)).count()));
    t.push_str(&format!("sockets with drops: {}\n", socks.iter().filter(|x| x.drops > 0).count()));
    let d: Vec<String> = report["dns_sockets"].as_array().unwrap().iter()
        .map(|x| format!("{} x{}", s(x, "server"), u(x, "count"))).collect();
    t.push_str(&format!("connected sockets to :53: {}\n", if d.is_empty() { "none".to_string() } else { d.join(", ") }));
    Ok((report, t))
}

// ------------------------------------------------------------------ text inputs

pub fn parse_resolv(lines: &[String]) -> Value {
    let mut ns = Vec::new();
    let (mut timeout, mut attempts, mut found) = (5u64, 2u64, false);
    for line in lines {
        let body = line.split('#').next().unwrap_or("");
        let fl: Vec<&str> = body.split_whitespace().collect();
        if fl.is_empty() {
            continue;
        }
        if fl[0] == "nameserver" && fl.len() > 1 {
            ns.push(fl[1].to_string());
            found = true;
        } else if fl[0] == "options" {
            for o in &fl[1..] {
                let (k, v) = o.split_once(':').unwrap_or((o, ""));
                if !v.is_empty() && v.chars().all(|c| c.is_ascii_digit()) {
                    if k == "timeout" {
                        timeout = v.parse().unwrap_or(timeout);
                    } else if k == "attempts" {
                        attempts = v.parse().unwrap_or(attempts);
                    }
                }
            }
        }
    }
    if !found {
        return Value::Null;
    }
    json!({"nameservers": ns, "timeout": timeout, "attempts": attempts})
}

/// "HH:MM:SS[.frac] IP " -> seconds of day
fn tcpdump_head(line: &str) -> Option<f64> {
    let b = line.as_bytes();
    if b.len() < 12 || b[2] != b':' || b[5] != b':' {
        return None;
    }
    let digits = |r: std::ops::Range<usize>| line.get(r.clone()).filter(|x| x.chars().all(|c| c.is_ascii_digit()));
    let hh: f64 = digits(0..2)?.parse().ok()?;
    let mm: f64 = digits(3..5)?.parse().ok()?;
    let sp = line.find(' ')?;
    let sec = &line[6..sp];
    if !line[sp..].starts_with(" IP ") || sec.len() < 2 || !sec[..2].chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    if sec.len() > 2 && (!sec[2..].starts_with('.') || !sec[3..].chars().all(|c| c.is_ascii_digit()) || sec.len() == 3) {
        return None;
    }
    let ss: f64 = sec.parse().ok()?;
    Some(hh * 3600.0 + mm * 60.0 + ss)
}

fn ip_port(tok: &str) -> Option<(String, String)> {
    let p: Vec<&str> = tok.split('.').collect();
    if p.len() != 5 || !p.iter().all(|x| !x.is_empty() && x.chars().all(|c| c.is_ascii_digit())) {
        return None;
    }
    Some((p[..4].join("."), p[4].to_string()))
}

struct DnsPkt {
    src: String,
    sport: String,
    dst: String,
    dport: String,
    id: String,
    rcode: Option<String>,
}

fn tcpdump_dns(line: &str) -> Option<DnsPkt> {
    let tok: Vec<&str> = line.split_whitespace().collect();
    for i in 1..tok.len().saturating_sub(1) {
        if tok[i] != ">" || !tok[i + 1].ends_with(':') {
            continue;
        }
        let (a, b) = match (ip_port(tok[i - 1]), ip_port(&tok[i + 1][..tok[i + 1].len() - 1])) {
            (Some(a), Some(b)) => (a, b),
            _ => continue,
        };
        let mut j = i + 2;
        if j < tok.len() && tok[j].starts_with('[') {
            while j < tok.len() && !tok[j].ends_with(']') {
                j += 1;
            }
            j += 1;
        }
        if j >= tok.len() {
            return None;
        }
        let qid = tok[j].trim_end_matches(|c| "+*%-$|".contains(c));
        if qid.is_empty() || !qid.chars().all(|c| c.is_ascii_digit()) {
            return None;
        }
        let mut rcode = None;
        for k in (j + 1)..tok.len().min(j + 6) {
            let t = tok[k];
            if RCODES.contains(&t) {
                rcode = Some(t.to_string());
            } else if t.len() > 1
                && t.ends_with('?')
                && t[..t.len() - 1].chars().all(|c| c.is_ascii_alphanumeric())
                && t[..t.len() - 1].chars().any(|c| c.is_ascii_uppercase())
                && !t[..t.len() - 1].chars().any(|c| c.is_ascii_lowercase())
                && k + 1 < tok.len()
            {
                break;
            } else if t != "q:" {
                break;
            }
        }
        return Some(DnsPkt { src: a.0, sport: a.1, dst: b.0, dport: b.1, id: qid.to_string(), rcode });
    }
    None
}

fn getent_real(line: &str) -> Option<f64> {
    let rest = line.trim().strip_prefix("real")?;
    if !rest.starts_with(|c: char| c.is_whitespace()) {
        return None;
    }
    let rest = rest.trim_start();
    let (m, sec) = rest.split_once('m')?;
    if m.is_empty() || !m.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    let num: String = sec.chars().take_while(|c| c.is_ascii_digit() || *c == '.').collect();
    if num.is_empty() || !sec[num.len()..].starts_with('s') {
        return None;
    }
    Some(r2(m.parse::<f64>().ok()? * 60.0 + num.parse::<f64>().ok()?))
}

pub fn parse_dns_text(lines: &[String]) -> (Value, Vec<f64>, Value) {
    let mut joined: Vec<String> = Vec::new();
    for line in lines {
        let cont = line.starts_with(' ') || line.starts_with('\t');
        if cont && joined.last().map(|l| tcpdump_head(l).is_some()).unwrap_or(false) {
            let l = joined.last_mut().unwrap();
            l.push(' ');
            l.push_str(line.trim());
        } else {
            joined.push(line.clone());
        }
    }
    // (server, sport, id) -> (t, answered)
    let mut queries: HashMap<(String, String, String), (f64, String, bool)> = HashMap::new();
    let mut replies: Vec<(f64, String, (String, String, String), String)> = Vec::new();
    let mut getent = Vec::new();
    for line in &joined {
        if let Some(g) = getent_real(line) {
            getent.push(g);
            continue;
        }
        let t = match tcpdump_head(line) {
            Some(t) => t,
            None => continue,
        };
        let q = match tcpdump_dns(line) {
            Some(q) => q,
            None => continue,
        };
        if q.dport == "53" {
            queries.insert((q.dst.clone(), q.sport.clone(), q.id.clone()), (t, q.dst.clone(), false));
        } else if q.sport == "53" {
            replies.push((t, q.src.clone(), (q.src.clone(), q.dport.clone(), q.id.clone()),
                q.rcode.unwrap_or_else(|| "NoError".into())));
        }
    }
    #[derive(Default)]
    struct Srv {
        queries: u64,
        replies: u64,
        rcodes: BTreeMap<String, u64>,
        lat: Vec<f64>,
        unanswered: u64,
    }
    let mut per: BTreeMap<String, Srv> = BTreeMap::new();
    for (_, server, _) in queries.values() {
        per.entry(server.clone()).or_default().queries += 1;
    }
    for (t, server, key, rcode) in &replies {
        let sv = per.entry(server.clone()).or_default();
        sv.replies += 1;
        *sv.rcodes.entry(rcode.clone()).or_default() += 1;
        if let Some(q) = queries.get_mut(key) {
            if !q.2 {
                q.2 = true;
                sv.lat.push(t - q.0);
            }
        }
    }
    for (_, server, answered) in queries.values() {
        if !answered {
            per.entry(server.clone()).or_default().unanswered += 1;
        }
    }
    let mut servers = Vec::new();
    for (name, sv) in per {
        let median_ms = r2(median_f(&sv.lat) * 1000.0);
        let max_ms = if sv.lat.is_empty() { 0.0 } else { r2(sv.lat.iter().cloned().fold(f64::MIN, f64::max) * 1000.0) };
        let failed: u64 = sv.rcodes.iter().filter(|(k, _)| *k == "ServFail" || *k == "Refused").map(|(_, v)| *v).sum();
        let bad = sv.queries >= 3
            && ((failed * 2 >= sv.replies.max(1) && sv.replies > 0)
                || sv.unanswered * 5 >= sv.queries
                || median_ms >= 1000.0);
        servers.push(json!({
            "server": name, "queries": sv.queries, "replies": sv.replies, "rcodes": sv.rcodes,
            "median_ms": median_ms, "max_ms": max_ms, "unanswered": sv.unanswered, "bad": bad,
        }));
    }
    (Value::Array(servers), getent, parse_resolv(lines))
}

/// "[pid  N] HH:MM:SS.us body" or "N  HH:MM:SS.us body" or "HH:MM:SS.us body" -> (pid, body)
fn strace_split(line: &str) -> Option<(String, String)> {
    let mut rest = line.trim();
    let mut pid = String::new();
    if let Some(r) = rest.strip_prefix("[pid") {
        let r = r.trim_start();
        let end = r.find(']')?;
        let p = r[..end].trim();
        if p.is_empty() || !p.chars().all(|c| c.is_ascii_digit()) {
            return None;
        }
        pid = p.to_string();
        rest = r[end + 1..].trim_start();
    } else {
        let first: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
        let after = &rest[first.len()..];
        if !first.is_empty() && after.starts_with(char::is_whitespace) {
            let a = after.trim_start();
            if a.len() >= 8 && a.as_bytes()[2] == b':' {
                pid = first;
                rest = a;
            }
        }
    }
    let b = rest.as_bytes();
    if b.len() < 9 || b[2] != b':' || b[5] != b':' {
        return None;
    }
    let sp = rest.find(char::is_whitespace)?;
    let ts = &rest[..sp];
    if !ts.chars().all(|c| c.is_ascii_digit() || c == ':' || c == '.') {
        return None;
    }
    if pid.is_empty() {
        pid = "0".into();
    }
    Some((pid, rest[sp..].trim_start().to_string()))
}

fn strace_dur(body: &str) -> Option<f64> {
    let b = body.trim_end();
    let b = b.strip_suffix('>')?;
    let i = b.rfind('<')?;
    let n = &b[i + 1..];
    let (a, c) = n.split_once('.')?;
    if a.is_empty() || c.is_empty() || !a.chars().all(|x| x.is_ascii_digit()) || !c.chars().all(|x| x.is_ascii_digit()) {
        return None;
    }
    n.parse().ok()
}

fn digits_prefix(s: &str) -> Option<&str> {
    let n = s.chars().take_while(|c| c.is_ascii_digit()).count();
    if n == 0 { None } else { Some(&s[..n]) }
}

pub fn parse_strace(lines: &[String]) -> Value {
    let mut fd_server: HashMap<(String, String), String> = HashMap::new();
    let mut pending: BTreeMap<String, String> = BTreeMap::new();
    let mut blocks: Vec<(String, f64)> = Vec::new();
    for line in lines {
        let (pid, body) = match strace_split(line) {
            Some(x) => x,
            None => continue,
        };
        if let Some(r) = body.strip_prefix("connect(") {
            let fd = match digits_prefix(r) {
                Some(fd) => fd.to_string(),
                None => continue,
            };
            let r = &r[fd.len()..];
            let port = r.strip_prefix(", {sa_family=AF_INET, sin_port=htons(").and_then(digits_prefix);
            let ip = r.find("sin_addr=inet_addr(\"").map(|i| &r[i + 20..]).and_then(|x| x.split('"').next());
            match (port, ip) {
                (Some(p), Some(ip)) if r.contains(&format!("htons({}), sin_addr=inet_addr(\"{}\")", p, ip))
                    && !ip.is_empty() && ip.chars().all(|c| c.is_ascii_digit() || c == '.') =>
                {
                    if p == "53" {
                        fd_server.insert((pid.clone(), fd), ip.to_string());
                    } else {
                        fd_server.remove(&(pid.clone(), fd));
                    }
                }
                _ => {}
            }
            continue;
        }
        if let Some(r) = body.strip_prefix("poll([{fd=") {
            let fd = match digits_prefix(r) {
                Some(fd) => fd.to_string(),
                None => continue,
            };
            let t = r[fd.len()..].strip_prefix(", events=POLLIN}], 1, ").and_then(digits_prefix);
            let t: u64 = match t.and_then(|x| x.parse().ok()) {
                Some(t) => t,
                None => continue,
            };
            if t < 1000 {
                continue;
            }
            let server = match fd_server.get(&(pid.clone(), fd)) {
                Some(sv) => sv.clone(),
                None => continue,
            };
            if body.contains("<unfinished") {
                pending.insert(pid, server);
                continue;
            }
            if let Some(d) = strace_dur(&body) {
                if d >= 1.0 {
                    blocks.push((server, d));
                }
            }
            continue;
        }
        if body.starts_with("<... poll resumed>") {
            if let Some(server) = pending.remove(&pid) {
                if let Some(d) = strace_dur(&body) {
                    if d >= 1.0 {
                        blocks.push((server, d));
                    }
                }
            }
        }
    }
    for (_, server) in pending {
        blocks.push((server, -1.0));
    }
    let mut per: BTreeMap<String, (u64, f64, u64)> = BTreeMap::new();
    for (server, dur) in blocks {
        let e = per.entry(server).or_insert((0, 0.0, 0));
        e.0 += 1;
        if dur < 0.0 {
            e.2 += 1;
        } else {
            e.1 = e.1.max(r2(dur));
        }
    }
    Value::Array(per.into_iter().map(|(k, (c, m, un))| json!({"server": k, "count": c, "max_s": m, "unfinished": un})).collect())
}

/// "[date time P123-T456] [thread] L: message" -> (pid, thread, message)
fn nimble_line(line: &str) -> Option<(String, String, String)> {
    let r = line.strip_prefix('[')?;
    let end = r.find(']')?;
    let head: Vec<&str> = r[..end].split(' ').collect();
    if head.len() != 3 || head.iter().any(|x| x.is_empty()) {
        return None;
    }
    let (p, t) = head[2].strip_prefix('P')?.split_once("-T")?;
    if p.is_empty() || t.is_empty() || !p.chars().all(|c| c.is_ascii_digit()) || !t.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    let r = r[end + 1..].strip_prefix(" [")?;
    let tend = r.find(']')?;
    if tend == 0 {
        return None;
    }
    let thread = &r[..tend];
    let r = r[tend + 1..].strip_prefix(' ')?;
    let (lvl, msg) = r.split_once(": ")?;
    if lvl.chars().count() != 1 || !lvl.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
        return None;
    }
    Some((p.to_string(), thread.to_string(), msg.to_string()))
}

fn url_host(msg: &str) -> Option<String> {
    let i = msg.find("url='")?;
    let r = &msg[i + 5..];
    let (scheme, rest) = r.split_once("://")?;
    let mut ch = scheme.chars();
    if !ch.next()?.is_ascii_alphabetic() || !ch.all(|c| c.is_ascii_alphanumeric() || "+.-".contains(c)) {
        return None;
    }
    let host: String = rest.chars().take_while(|c| !"/:'?".contains(*c)).collect();
    if host.is_empty() { None } else { Some(host) }
}

fn resolve_host(msg: &str) -> String {
    let i = match msg.find("failed to resolve") {
        Some(i) => i,
        None => return "?".into(),
    };
    let mut r = &msg[i + 17..];
    if let Some(x) = r.strip_prefix(" ip address from") {
        r = x;
    } else if let Some(x) = r.strip_prefix(" host=") {
        r = x;
    }
    r = r.strip_prefix(' ').unwrap_or(r);
    r = r.strip_prefix('\'').unwrap_or(r);
    let h: String = r.chars().take_while(|c| *c != '\'' && !c.is_whitespace()).collect();
    if h.is_empty() { "?".into() } else { h }
}

pub fn parse_nimble_log(lines: &[String], pid: Option<&str>) -> Value {
    let mut rows: Vec<(String, String, String)> = lines.iter().filter_map(|l| nimble_line(l)).collect();
    let used_pid = match pid {
        Some(p) if rows.iter().any(|r| r.0 == p) => {
            rows.retain(|r| r.0 == p);
            Value::String(p.to_string())
        }
        _ => Value::Null,
    };
    let mut hosts: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let mut failures: BTreeMap<String, u64> = BTreeMap::new();
    for (_, thread, msg) in &rows {
        if let Some(h) = url_host(msg) {
            hosts.entry(thread.clone()).or_default().insert(h);
        }
        if msg.contains("failed to resolve") {
            *failures.entry(resolve_host(msg)).or_default() += 1;
        }
    }
    json!({"pid": used_pid, "lines": rows.len(), "thread_hosts": hosts, "resolve_failures": failures})
}

// ------------------------------------------------------------------ nimdiag directory

struct Loaded {
    streams: Vec<(String, Vec<Value>)>,
    sysrec: Vec<Value>,
    threads: Vec<Value>,
    events: Vec<Value>,
    snaps: BTreeMap<String, Value>,
    resolv: Value,
}

fn load_dir(d: &Path) -> Result<Loaded, String> {
    let timeline = read_jsonl(&d.join("timeline.jsonl"));
    if timeline.is_empty() {
        return Err(format!("{} has no timeline.jsonl (not a nimdiag result directory?)", d.display()));
    }
    let mut streams: Vec<(String, Vec<Value>)> = Vec::new();
    let mut sysrec = Vec::new();
    for r in timeline {
        match s(&r, "type") {
            "stream" => {
                let name = s(&r, "name").to_string();
                match streams.iter_mut().find(|(n, _)| *n == name) {
                    Some((_, v)) => v.push(r),
                    None => streams.push((name, vec![r])),
                }
            }
            "sys" => sysrec.push(r),
            _ => {}
        }
    }
    let threads = read_jsonl(&d.join("threads.jsonl"));
    let mut files: Vec<_> = fs::read_dir(d.join("events"))
        .map(|rd| rd.flatten().map(|e| e.path()).collect())
        .unwrap_or_else(|_| Vec::new());
    files.retain(|p: &std::path::PathBuf| {
        let n = p.file_name().and_then(|x| x.to_str()).unwrap_or("");
        n.starts_with("event_") && n.ends_with(".json")
    });
    files.sort();
    let events: Vec<Value> = files
        .iter()
        .filter_map(|p| fs::read(p).ok().and_then(|b| serde_json::from_slice(&b).ok()))
        .collect();
    let mut snaps = BTreeMap::new();
    for tag in ["start", "end"] {
        let p = d.join("env").join(format!("proc_net_udp_{}.txt", tag));
        if p.is_file() {
            snaps.insert(tag.to_string(), dns_sockets(&parse_proc_udp(&read_lines(&p)?)));
        }
    }
    let p = d.join("env").join("resolv.conf.txt");
    let resolv = if p.is_file() { parse_resolv(&read_lines(&p)?) } else { Value::Null };
    Ok(Loaded { streams, sysrec, threads, events, snaps, resolv })
}

fn sock(r: &Value) -> &Value {
    r.get("sock").unwrap_or(&Value::Null)
}

fn stream_stats(rows: &[Value]) -> Map<String, Value> {
    let n = rows.len() as u64;
    let rxq: Vec<u64> = rows.iter().map(|r| u(sock(r), "rxq")).collect();
    let drops: Vec<u64> = rows.iter().map(|r| u(sock(r), "drops_d")).collect();
    let rxq_max = rxq.iter().cloned().max().unwrap_or(0);
    let total_drops: u64 = drops.iter().sum();
    let cc: u64 = rows.iter().map(|r| u(r, "cc_err")).sum();
    let rtp: u64 = rows.iter().map(|r| u(r, "rtp_seq_err")).sum();
    let sync: u64 = rows.iter().map(|r| u(r, "sync_err")).sum();
    let data_seconds = rows.iter().filter(|r| u(r, "pkts") > 0).count() as u64;
    let mut m = Map::new();
    m.insert("name".into(), json!(s(&rows[0], "name")));
    m.insert("group".into(), json!(s(&rows[0], "group")));
    m.insert("seconds".into(), json!(n));
    m.insert("drops".into(), json!(total_drops));
    m.insert("drop_seconds".into(), json!(drops.iter().filter(|x| **x > 0).count()));
    m.insert("rxq_max".into(), json!(rxq_max));
    m.insert("full_seconds".into(), json!(rxq.iter().filter(|q| rxq_max >= FULL_RXQ && **q * 10 >= rxq_max * 9).count()));
    m.insert("wire_mbps".into(), json!(r2(rows.iter().map(|r| f(r, "mbps")).sum::<f64>() / n.max(1) as f64)));
    m.insert("wire_pkts".into(), json!(rows.iter().map(|r| u(r, "pkts")).sum::<u64>()));
    m.insert("data_seconds".into(), json!(data_seconds));
    m.insert("cc_err".into(), json!(cc));
    m.insert("rtp_seq_err".into(), json!(rtp));
    m.insert("sync_err".into(), json!(sync));
    m.insert("wire_clean".into(), json!(sync == 0 && data_seconds * 100 >= n * 95 && (cc + rtp) * 100 <= total_drops));
    let mut drains = Vec::new();
    if rxq_max >= FULL_RXQ {
        for i in 1..rxq.len() {
            if rxq[i - 1] * 10 >= rxq_max * 9 && rxq[i] * 10 <= rxq_max {
                drains.push(i as i64);
            }
        }
    }
    let iv: Vec<i64> = drains.windows(2).map(|w| w[1] - w[0]).collect();
    let (mut p, mut share) = (0i64, 0.0f64);
    if iv.len() >= 3 {
        let mut c: BTreeMap<i64, u64> = BTreeMap::new();
        for x in &iv {
            *c.entry(*x).or_default() += 1;
        }
        let mut cv: Vec<(i64, u64)> = c.into_iter().collect();
        cv.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
        p = cv[0].0;
        let ok = iv
            .iter()
            .filter(|x| p >= 2 && ((**x - p * ((**x as f64 / p as f64).round() as i64).max(1)).abs() <= 1))
            .count();
        share = r2(ok as f64 / iv.len() as f64);
    }
    m.insert("drains".into(), json!(drains.len()));
    m.insert("period_s".into(), json!(p));
    m.insert("period_share".into(), json!(share));
    m.insert("periodic".into(), json!(p >= 2 && share >= 0.8));
    m
}

/// The thread that reads this socket: many wakeups per second while the queue is
/// drained, (almost) none while it is full. threads.jsonl rolls up per second on the
/// sampler's own clock and may trail the socket counters by one record, so lags 0
/// and +1 are both tried.
fn reader_thread(rows: &[Value], rxq_max: u64, threads: &[Value]) -> Value {
    let mut nz: Vec<u64> = rows.iter().map(|r| u(sock(r), "drops_d")).filter(|d| *d > 0).collect();
    if nz.is_empty() || rxq_max < FULL_RXQ {
        return Value::Null;
    }
    nz.sort();
    let full_rate = nz[nz.len() / 2];
    let label: Vec<Option<bool>> = rows
        .iter()
        .map(|r| {
            let (dd, q) = (u(sock(r), "drops_d"), u(sock(r), "rxq"));
            if dd == 0 && q * 2 < rxq_max {
                Some(true) // read the whole second
            } else if dd * 10 >= full_rate * 9 && q * 10 >= rxq_max * 9 {
                Some(false) // stalled the whole second
            } else {
                None // transition
            }
        })
        .collect();
    let pos: HashMap<String, usize> = rows.iter().enumerate().map(|(i, r)| (sec_key(s(r, "t")), i)).collect();
    let mut order: Vec<(String, i64)> = Vec::new();
    let mut series: HashMap<(String, i64), HashMap<usize, &Value>> = HashMap::new();
    for rec in threads {
        let i = match pos.get(&sec_key(s(rec, "t"))) {
            Some(i) => *i,
            None => continue,
        };
        for x in rec.get("threads").and_then(|v| v.as_array()).map(|v| v.as_slice()).unwrap_or(&[]) {
            let key = (s(x, "name").to_string(), x.get("tid").and_then(|v| v.as_i64()).unwrap_or(0));
            if !series.contains_key(&key) {
                order.push(key.clone());
            }
            series.entry(key).or_default().insert(i, x);
        }
    }
    // (name, tid, lag, read_vcsw, stall_vcsw, poll_share, camera)
    let mut cands: Vec<(String, i64, usize, f64, f64, f64, bool)> = Vec::new();
    for key in &order {
        let sv = &series[key];
        for lag in [0usize, 1] {
            let (mut read, mut stall): (Vec<&Value>, Vec<&Value>) = (Vec::new(), Vec::new());
            for (i, a) in label.iter().enumerate() {
                if let (Some(a), Some(x)) = (a, sv.get(&(i + lag))) {
                    if *a { read.push(x) } else { stall.push(x) }
                }
            }
            if read.len() < 5 || stall.len() < 5 {
                continue;
            }
            let mr = read.iter().map(|x| u(x, "vcsw_d")).sum::<u64>() as f64 / read.len() as f64;
            let ms = stall.iter().map(|x| u(x, "vcsw_d")).sum::<u64>() as f64 / stall.len() as f64;
            if mr <= 0.0 || ms > 0.2 * mr {
                continue;
            }
            let poll = stall.iter().filter(|x| POLL.contains(&sval(x, "last_syscall").as_str())).count();
            cands.push((key.0.clone(), key.1, lag, r2(mr), r2(ms), r2(poll as f64 / stall.len() as f64),
                key.0.starts_with("mpeg2tscamera")));
        }
    }
    let mut pool: Vec<_> = cands.iter().filter(|c| c.6).cloned().collect();
    if pool.is_empty() {
        pool = cands;
    }
    pool.sort_by(|a, b| {
        (b.3 - b.4).partial_cmp(&(a.3 - a.4)).unwrap_or(std::cmp::Ordering::Equal).then(a.1.cmp(&b.1)).then(a.2.cmp(&b.2))
    });
    match pool.first() {
        Some(b) => json!({"name": b.0, "tid": b.1, "lag": b.2, "read_vcsw": b.3, "stall_vcsw": b.4, "poll_share": b.5}),
        None => Value::Null,
    }
}

type SampleSet = BTreeSet<(usize, usize)>;

fn blocked_threads(events: &[Value], threads: &[Value]) -> (Vec<Map<String, Value>>, HashMap<i64, SampleSet>, &'static str) {
    let mut runs: BTreeMap<i64, (u64, u64)> = BTreeMap::new();
    let mut sets: HashMap<i64, SampleSet> = HashMap::new();
    let mut names: HashMap<i64, String> = HashMap::new();
    let arr = |v: &Value, k: &str| -> Vec<Value> { v.get(k).and_then(|x| x.as_array()).cloned().unwrap_or_default() };
    let have_events = events.iter().any(|e| !arr(e, "nimble_threads").is_empty());
    let source;
    if have_events {
        source = "events";
        for (ei, e) in events.iter().enumerate() {
            let samples = arr(e, "nimble_threads");
            let times: Vec<u64> = samples.iter().map(|x| ms_of_day(s(x, "t"))).collect();
            let diffs: Vec<u64> = times.windows(2).filter(|w| w[1] > w[0]).map(|w| w[1] - w[0]).collect();
            let mut step = median_u(&diffs);
            if step == 0 {
                step = 200;
            }
            let mut cur: BTreeMap<i64, u64> = BTreeMap::new();
            for (si, smp) in samples.iter().enumerate() {
                let mut seen = BTreeSet::new();
                for x in arr(smp, "threads") {
                    let tid = x.get("tid").and_then(|v| v.as_i64()).unwrap_or(0);
                    names.insert(tid, s(&x, "name").to_string());
                    seen.insert(tid);
                    if POLL.contains(&sval(&x, "syscall").as_str()) && u(&x, "vcsw_d") == 0 && f(&x, "cpu_pct") < 1.0 {
                        let c = cur.entry(tid).or_default();
                        *c += 1;
                        let c = *c;
                        sets.entry(tid).or_default().insert((ei, si));
                        let r = runs.entry(tid).or_default();
                        r.0 = r.0.max(c * step);
                    } else {
                        let c = cur.get(&tid).copied().unwrap_or(0);
                        if c > 0 && c * step >= 1000 {
                            runs.entry(tid).or_default().1 += 1;
                        }
                        cur.insert(tid, 0);
                    }
                }
                for (tid, c) in cur.iter_mut() {
                    if !seen.contains(tid) {
                        if *c * step >= 1000 {
                            runs.entry(*tid).or_default().1 += 1;
                        }
                        *c = 0;
                    }
                }
            }
            for (tid, c) in cur {
                if c * step >= 1000 {
                    runs.entry(tid).or_default().1 += 1;
                }
            }
        }
    } else if !threads.is_empty() {
        source = "seconds";
        let mut cur: BTreeMap<i64, u64> = BTreeMap::new();
        for (si, rec) in threads.iter().enumerate() {
            let mut seen = BTreeSet::new();
            for x in arr(rec, "threads") {
                let tid = x.get("tid").and_then(|v| v.as_i64()).unwrap_or(0);
                names.insert(tid, s(&x, "name").to_string());
                seen.insert(tid);
                if POLL.contains(&sval(&x, "last_syscall").as_str()) && u(&x, "vcsw_d") == 0 {
                    let c = cur.entry(tid).or_default();
                    *c += 1;
                    let c = *c;
                    sets.entry(tid).or_default().insert((0, si));
                    let r = runs.entry(tid).or_default();
                    r.0 = r.0.max(c * 1000);
                } else {
                    if cur.get(&tid).copied().unwrap_or(0) >= 2 {
                        runs.entry(tid).or_default().1 += 1;
                    }
                    cur.insert(tid, 0);
                }
            }
            for (tid, c) in cur.iter_mut() {
                if !seen.contains(tid) {
                    if *c >= 2 {
                        runs.entry(*tid).or_default().1 += 1;
                    }
                    *c = 0;
                }
            }
        }
        for (tid, c) in cur {
            if c >= 2 {
                runs.entry(tid).or_default().1 += 1;
            }
        }
    } else {
        source = "none";
    }
    let limit = if source == "events" { 1000 } else { 2000 };
    let mut out: Vec<Map<String, Value>> = runs
        .iter()
        .filter(|(_, r)| r.0 >= limit)
        .map(|(t, r)| {
            let mut m = Map::new();
            m.insert("name".into(), json!(names[t]));
            m.insert("tid".into(), json!(t));
            m.insert("max_stall_ms".into(), json!(r.0));
            m.insert("stall_runs".into(), json!(r.1));
            m
        })
        .collect();
    out.sort_by(|a, b| {
        a["name"].as_str().cmp(&b["name"].as_str()).then(a["tid"].as_i64().cmp(&b["tid"].as_i64()))
    });
    (out, sets, source)
}

/// (name, average CPU %) of the busiest mpeg2tscamera thread in threads.jsonl.
fn busiest_camera(threads: &[Value]) -> Option<(String, f64)> {
    let mut acc: BTreeMap<(String, i64), (f64, u64)> = BTreeMap::new();
    for rec in threads {
        for x in rec.get("threads").and_then(|v| v.as_array()).map(|v| v.as_slice()).unwrap_or(&[]) {
            let name = s(x, "name");
            if name.starts_with("mpeg2tscamera") {
                let a = acc.entry((name.to_string(), x.get("tid").and_then(|v| v.as_i64()).unwrap_or(0))).or_default();
                a.0 += f(x, "cpu_pct");
                a.1 += 1;
            }
        }
    }
    let mut v: Vec<((String, i64), f64)> = acc.into_iter().map(|(k, (c, n))| (k, c / n as f64)).collect();
    v.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal).then(a.0 .1.cmp(&b.0 .1)));
    v.into_iter().next().map(|(k, c)| (k.0, c))
}

fn host_counters(sysrec: &[Value]) -> (Value, u64) {
    let cap: u64 = sysrec.iter().map(|r| u(r, "capture_drops_d")).sum();
    let soft: u64 = sysrec.iter().map(|r| u(r, "softnet_dropped_d")).sum();
    let mut nic = 0u64;
    for r in sysrec {
        if let Some(Value::Object(m)) = r.get("nic") {
            for (_, d) in m {
                if let Value::Object(d) = d {
                    for (k, v) in d {
                        if k != "rx_packets" && k != "rx_bytes" && k != "multicast" {
                            nic += v.as_u64().unwrap_or(0);
                        }
                    }
                }
            }
        }
    }
    let live = sysrec.iter().map(|r| u(r, "dns_socks")).max().unwrap_or(0);
    (json!({"capture_drops": cap, "softnet_dropped": soft, "nic_errors": nic,
            "clean": cap == 0 && soft == 0 && nic == 0}), live)
}

fn nimble_pid(streams: &[(String, Vec<Value>)]) -> Option<String> {
    for (_, rows) in streams {
        for r in rows {
            if let Some(owners) = sock(r).get("owners").and_then(|v| v.as_array()) {
                for o in owners {
                    let o = o.as_str().unwrap_or("");
                    if let Some(i) = o.rfind(':') {
                        let p = &o[i + 1..];
                        if !p.is_empty() && p.chars().all(|c| c.is_ascii_digit()) {
                            return Some(p.to_string());
                        }
                    }
                }
            }
        }
    }
    None
}

pub struct Inputs<'a> {
    pub dir: &'a Path,
    pub stream: Option<&'a str>,
    pub dns: Option<&'a Path>,
    pub strace: Option<&'a Path>,
    pub nimble_log: Option<&'a Path>,
}

pub fn analyze(inp: &Inputs) -> Result<Value, String> {
    let ld = load_dir(inp.dir)?;
    let (host, live_dns) = host_counters(&ld.sysrec);
    let (mut blocked, sets, source) = blocked_threads(&ld.events, &ld.threads);

    let mut resolv = ld.resolv.clone();
    let (mut servers, mut getent) = (Value::Array(vec![]), Vec::new());
    if let Some(p) = inp.dns {
        let (sv, g, rv) = parse_dns_text(&read_lines(p)?);
        servers = sv;
        getent = g;
        if resolv.is_null() {
            resolv = rv;
        }
    }
    let strace = match inp.strace {
        Some(p) => parse_strace(&read_lines(p)?),
        None => Value::Array(vec![]),
    };
    let log = match inp.nimble_log {
        Some(p) => parse_nimble_log(&read_lines(p)?, nimble_pid(&ld.streams).as_deref()),
        None => Value::Null,
    };
    let snap = |k: &str| ld.snaps.get(k).cloned().unwrap_or(Value::Array(vec![]));
    let count = |v: &Value| v.as_array().map(|a| a.iter().map(|x| u(x, "count")).sum::<u64>()).unwrap_or(0);
    let sock_max = count(&snap("start")).max(count(&snap("end"))).max(live_dns);
    let bad: Vec<String> = servers.as_array().unwrap().iter()
        .filter(|x| x["bad"].as_bool() == Some(true)).map(|x| s(x, "server").to_string()).collect();
    let timeout = resolv.get("timeout").and_then(|v| v.as_u64()).unwrap_or(5);

    let mut evidence: Vec<String> = Vec::new();
    let mut missing: Vec<String> = Vec::new();
    let mut dns_evidence = false;
    if sock_max > 0 {
        dns_evidence = true;
        evidence.push(format!("up to {} connected UDP sockets to port 53 at once (resolver queries in flight)", sock_max));
    }
    if !bad.is_empty() {
        dns_evidence = true;
        evidence.push(format!("slow or failing nameserver(s) in the DNS capture: {}", bad.join(", ")));
    }
    let slow: Vec<String> = getent.iter().filter(|g| **g >= 1.0).map(|g| format!("{:.2}", g)).collect();
    if !slow.is_empty() {
        dns_evidence = true;
        evidence.push(format!("getent lookups took {} s", slow.join(", ")));
    }
    let sb = strace.as_array().unwrap();
    if !sb.is_empty() {
        dns_evidence = true;
        evidence.push(format!("strace: {} resolver poll() call(s) blocked >= 1 s ({})",
            sb.iter().map(|x| u(x, "count")).sum::<u64>(),
            sb.iter().map(|x| s(x, "server").to_string()).collect::<Vec<_>>().join(", ")));
    }
    if let Some(Value::Object(rf)) = log.get("resolve_failures") {
        if !rf.is_empty() {
            dns_evidence = true;
            evidence.push(format!("Nimble log reports failed resolution of {}",
                rf.keys().cloned().collect::<Vec<_>>().join(", ")));
        }
    }
    if inp.dns.is_none() {
        missing.push("DNS check: resolv.conf, `time getent ahosts <pull host>` and a 30 s tcpdump of port 53".into());
    }
    if inp.strace.is_none() {
        missing.push("optional: strace of the blocked thread ids (connect to :53 followed by a long poll)".into());
    }
    if inp.nimble_log.is_none() {
        missing.push("Nimble log with the thread name -> pull URL mapping".into());
    }
    if source == "none" {
        missing.push("thread samples (nimdiag could not see the Nimble process)".into());
    }

    let names: Vec<String> = match inp.stream {
        Some(n) => vec![n.to_string()],
        None => ld.streams.iter().map(|(n, _)| n.clone()).collect(),
    };
    let mut results: Vec<Map<String, Value>> = Vec::new();
    let host_clean = host["clean"].as_bool() == Some(true);
    for name in names {
        let rows = match ld.streams.iter().find(|(n, _)| *n == name) {
            Some((_, r)) => r,
            None => return Err(format!("stream not found: {}", name)),
        };
        let mut st = stream_stats(rows);
        let g = |k: &str| st[k].as_u64().unwrap_or(0);
        let (drops, rxq_max) = (g("drops"), g("rxq_max"));
        if inp.stream.is_none() && drops == 0 && rxq_max < FULL_RXQ {
            continue;
        }
        let reader = reader_thread(rows, rxq_max, &ld.threads);
        let reader_tid = reader.get("tid").and_then(|v| v.as_i64());
        let reader_blocked = reader_tid.and_then(|t| blocked.iter().find(|b| b["tid"].as_i64() == Some(t)).cloned());
        let mut reasons: Vec<String> = Vec::new();
        let periodic = st["periodic"].as_bool() == Some(true);
        let verdict;
        if drops == 0 {
            verdict = "no-drops";
            reasons.push("no socket drops during the run".into());
        } else if st["wire_clean"].as_bool() != Some(true) {
            verdict = "not-matched";
            reasons.push(format!(
                "the wire itself shows loss or gaps (CC/RTP errors {}, sync errors {}, data in {} of {} s): look upstream first",
                g("cc_err") + g("rtp_seq_err"), g("sync_err"), g("data_seconds"), g("seconds")));
        } else if !host_clean {
            verdict = "not-matched";
            reasons.push(format!("drops before the socket (capture {}, softnet {}, NIC {}): host or NIC problem",
                u(&host, "capture_drops"), u(&host, "softnet_dropped"), u(&host, "nic_errors")));
        } else {
            reasons.push(format!("kernel dropped {} datagrams at Nimble's socket in {} of {} s while the wire was clean",
                drops, g("drop_seconds"), g("seconds")));
            if periodic {
                reasons.push(format!("queue fills and drains with a stable period of {} s ({}% of cycles)",
                    g("period_s"), (st["period_share"].as_f64().unwrap_or(0.0) * 100.0).round() as i64));
            }
            if !reader.is_null() {
                reasons.push(format!("reader thread {} (tid {}) wakes {:.0} times/s while the queue drains and {:.0}/s while it is full",
                    s(&reader, "name"), reader_tid.unwrap_or(0), f(&reader, "read_vcsw"), f(&reader, "stall_vcsw")));
            }
            if let Some(rb) = &reader_blocked {
                reasons.push(format!("the reader sits in poll() with no context switches for up to {:.1} s",
                    rb["max_stall_ms"].as_u64().unwrap_or(0) as f64 / 1000.0));
                let others: Vec<&Map<String, Value>> = blocked.iter().filter(|b| b["tid"].as_i64() != reader_tid).collect();
                if !others.is_empty() {
                    reasons.push(format!("{} other thread(s) block the same way: {}", others.len(),
                        others.iter().map(|b| b["name"].as_str().unwrap_or("").to_string()).collect::<Vec<_>>().join(", ")));
                }
            }
            let matches_timeout = reader_blocked.as_ref()
                .map(|b| b["max_stall_ms"].as_u64().unwrap_or(0) >= timeout * 800).unwrap_or(false);
            if reader_blocked.is_some() && dns_evidence && (periodic || matches_timeout) {
                verdict = "confirmed";
            } else if reader_blocked.is_some() {
                verdict = "probable";
                if !dns_evidence {
                    reasons.push("no DNS evidence yet: collect it to confirm the resolver as the cause".into());
                }
            } else if reader.is_null() && source == "none" && periodic && dns_evidence {
                verdict = "probable";
                reasons.push("no thread samples; periodic stalls plus DNS evidence fit the pattern".into());
            } else {
                verdict = "not-matched";
                if !reader.is_null() && f(&reader, "poll_share") < 0.5 {
                    reasons.push("the reader is not blocked in poll() while the queue is full: look at CPU load or thread distribution instead".into());
                } else if reader.is_null() && g("drains") == 0 {
                    let busy = busiest_camera(&ld.threads)
                        .map(|(n, c)| format!(" (busiest: {}, {:.0}% CPU on average)", n, c))
                        .unwrap_or_default();
                    reasons.push(format!("the queue never drains: the reader is permanently behind, not periodically blocked; check the CPU use of the ingest threads{}", busy));
                } else if reader.is_null() {
                    reasons.push("no thread follows the reading phases".into());
                }
            }
        }
        st.insert("reader".into(), reader);
        st.insert("verdict".into(), json!(verdict));
        st.insert("reasons".into(), json!(reasons));
        results.push(st);
    }

    let order = ["confirmed", "probable", "not-matched", "no-drops"];
    let overall = results
        .iter()
        .map(|r| r["verdict"].as_str().unwrap_or("no-drops"))
        .min_by_key(|v| order.iter().position(|o| o == v).unwrap_or(3))
        .unwrap_or("no-drops");
    let affected: Vec<String> = results.iter()
        .filter(|r| matches!(r["verdict"].as_str(), Some("confirmed") | Some("probable")))
        .map(|r| r["name"].as_str().unwrap_or("").to_string()).collect();
    let mut mapped = Map::new();
    if let Some(Value::Object(th)) = log.get("thread_hosts") {
        for b in &blocked {
            if let Some(h) = th.get(b["name"].as_str().unwrap_or("")) {
                mapped.insert(b["name"].as_str().unwrap_or("").to_string(), h.clone());
            }
        }
    }
    let first_reader = results.first().and_then(|r| r["reader"].get("tid")).and_then(|v| v.as_i64());
    for b in blocked.iter_mut() {
        let tid = b["tid"].as_i64().unwrap_or(0);
        let ov = match (sets.get(&tid), first_reader.and_then(|t| sets.get(&t))) {
            (Some(a), Some(r)) => r2(a.intersection(r).count() as f64 / a.union(r).count().max(1) as f64),
            _ => 0.0,
        };
        b.insert("overlap_with_reader".into(), json!(ov));
    }
    let summary = match overall {
        "confirmed" => "an ingest thread is blocked in DNS resolution and stops reading its MPEG-TS inputs",
        "probable" => "an ingest thread stops reading its inputs while blocked in poll(); DNS is the likely cause",
        "not-matched" => "socket drops do not match the blocked-resolver pattern",
        _ => "no socket drops were recorded",
    };
    Ok(json!({
        "tool": "mpegts-ingest-getaddrinfo-stall", "version": VERSION, "kind": "analyze",
        "verdict": overall, "summary": summary, "affected_streams": affected,
        "streams": results,
        "threads": {"source": source, "blocked": blocked, "pull_hosts": mapped},
        "host": host,
        "dns": {"resolv": resolv, "sockets": {"start": snap("start"), "end": snap("end"), "max_live": live_dns},
                "servers": servers, "getent_s": getent, "strace_blocking": strace, "log": log},
        "evidence": evidence, "missing": missing,
    }))
}

pub fn report_text(r: &Value) -> String {
    let mut t = format!("verdict: {} - {}\n", s(r, "verdict"), s(r, "summary"));
    for st in r["streams"].as_array().unwrap() {
        let period = if u(st, "period_s") > 0 {
            format!("{} s ({}%)", u(st, "period_s"), (f(st, "period_share") * 100.0).round() as i64)
        } else {
            "-".into()
        };
        t.push_str(&format!("\n--- {} [{}]  {}\n", s(st, "name"), s(st, "group"), s(st, "verdict")));
        t.push_str(&format!("  wire {:.2} Mbps, drops {} in {}/{} s, max rx_queue {} B, full {} s, period {}\n",
            f(st, "wire_mbps"), u(st, "drops"), u(st, "drop_seconds"), u(st, "seconds"), u(st, "rxq_max"),
            u(st, "full_seconds"), period));
        for x in st["reasons"].as_array().unwrap() {
            t.push_str(&format!("  * {}\n", x.as_str().unwrap_or("")));
        }
    }
    let th = &r["threads"];
    let bl: Vec<String> = th["blocked"].as_array().unwrap().iter()
        .map(|b| format!("{}/{} {:.1}s x{}", s(b, "name"), b["tid"], u(b, "max_stall_ms") as f64 / 1000.0, u(b, "stall_runs")))
        .collect();
    t.push_str(&format!("\nblocked threads ({}): {}\n", s(th, "source"), if bl.is_empty() { "none".into() } else { bl.join(", ") }));
    if let Some(m) = th["pull_hosts"].as_object() {
        for (name, hosts) in m {
            let h: Vec<&str> = hosts.as_array().map(|a| a.iter().filter_map(|x| x.as_str()).collect()).unwrap_or_default();
            t.push_str(&format!("  {} pulls from {}\n", name, h.join(", ")));
        }
    }
    let d = &r["dns"];
    if !d["resolv"].is_null() {
        let ns: Vec<&str> = d["resolv"]["nameservers"].as_array().unwrap().iter().filter_map(|x| x.as_str()).collect();
        t.push_str(&format!("resolv.conf: nameservers {}, timeout {} s, attempts {}\n",
            ns.join(" "), u(&d["resolv"], "timeout"), u(&d["resolv"], "attempts")));
    }
    for sv in d["servers"].as_array().unwrap() {
        t.push_str(&format!("  dns {}: {} queries, {} replies {}, median {:.1} ms, max {:.1} ms, unanswered {}{}\n",
            s(sv, "server"), u(sv, "queries"), u(sv, "replies"), py_json(&sv["rcodes"]), f(sv, "median_ms"),
            f(sv, "max_ms"), u(sv, "unanswered"), if sv["bad"].as_bool() == Some(true) { "  <- BAD" } else { "" }));
    }
    let ev = r["evidence"].as_array().unwrap();
    if !ev.is_empty() {
        t.push_str("\nDNS evidence:\n");
        for e in ev {
            t.push_str(&format!("  * {}\n", e.as_str().unwrap_or("")));
        }
    }
    let ms = r["missing"].as_array().unwrap();
    if !ms.is_empty() {
        t.push_str("\nnot provided:\n");
        for e in ms {
            t.push_str(&format!("  - {}\n", e.as_str().unwrap_or("")));
        }
    }
    t
}

/// Compact JSON object with Python's json.dumps separators (", " and ": ").
fn py_json(v: &Value) -> String {
    match v.as_object() {
        Some(m) => format!("{{{}}}", m.iter().map(|(k, x)| format!("\"{}\": {}", k, x)).collect::<Vec<_>>().join(", ")),
        None => v.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lines(s: &str) -> Vec<String> {
        s.lines().map(|l| l.to_string()).collect()
    }

    #[test]
    fn proc_udp_decodes_little_endian_and_dns_sockets() {
        let t = lines("  sl  local_address rem_address   st tx_queue rx_queue tr tm->when retrnsmt   uid  timeout inode ref pointer drops\n\
   1: 0A00FCE9:04D2 00000000:0000 07 00000000:002006C0 00:00000000 00000000     0        0 1001 2 0000000000000000 5120\n\
   2: 0B6300C6:D431 350200C0:0035 01 00000000:00000000 00:00000000 00000000     0        0 1002 2 0000000000000000 0\n");
        let p = parse_proc_udp(&t);
        assert_eq!(p.len(), 2);
        assert_eq!(p[0].local, "233.252.0.10:1234");
        assert_eq!(p[0].rxq, 0x2006C0);
        assert_eq!(p[0].drops, 5120);
        assert_eq!(p[1].remote, "192.0.2.53:53");
        assert_eq!(dns_sockets(&p), json!([{"server": "192.0.2.53", "count": 1}]));
    }

    #[test]
    fn tcpdump_verbose_and_plain_forms() {
        let t = lines("12:00:00.000000 IP (tos 0x0, ttl 64, id 1, offset 0, flags [DF], proto UDP (17), length 60)\n    \
198.51.100.11.40000 > 192.0.2.53.53: [bad udp cksum 0x1 -> 0x2!] 100+ A? hls-a.example. (32)\n\
12:00:02.500000 IP (tos 0x0, ttl 63, id 2, offset 0, flags [none], proto UDP (17), length 60)\n    \
192.0.2.53.53 > 198.51.100.11.40000: [udp sum ok] 100 ServFail q: A? hls-a.example. 0/0/0 (32)\n\
12:00:03.000000 IP 198.51.100.11.40001 > 192.0.2.54.53: 101+ A? hls-a.example. (32)\n\
12:00:03.001000 IP 192.0.2.54.53 > 198.51.100.11.40001: 101 1/0/0 A 203.0.113.10 (48)\n\
real\t0m5.008s\n");
        let (sv, g, rv) = parse_dns_text(&t);
        let sv = sv.as_array().unwrap();
        assert_eq!(sv.len(), 2);
        assert_eq!(sv[0]["server"], "192.0.2.53");
        assert_eq!(sv[0]["rcodes"]["ServFail"], 1);
        assert_eq!(sv[0]["median_ms"], 2500.0);
        assert_eq!(sv[1]["rcodes"]["NoError"], 1);
        assert_eq!(sv[1]["median_ms"], 1.0);
        assert_eq!(g, vec![5.01]);
        assert!(rv.is_null());
    }

    #[test]
    fn strace_blocking_poll_on_dns_fd() {
        let t = lines("[pid  4242] 12:00:00.000100 connect(9, {sa_family=AF_INET, sin_port=htons(53), sin_addr=inet_addr(\"192.0.2.53\")}, 16) = 0 <0.000020>\n\
[pid  4242] 12:00:00.000300 poll([{fd=9, events=POLLIN}], 1, 5000) = 0 (Timeout) <5.004000>\n\
[pid  4243] 12:00:01.000000 connect(7, {sa_family=AF_INET, sin_port=htons(53), sin_addr=inet_addr(\"192.0.2.53\")}, 16) = 0 <0.000020>\n\
[pid  4243] 12:00:01.000100 poll([{fd=7, events=POLLIN}], 1, 5000 <unfinished ...>\n\
[pid  4243] 12:00:04.000100 <... poll resumed>) = 0 (Timeout) <3.000000>\n\
[pid  4242] 12:00:06.000000 poll([{fd=9, events=POLLIN}], 1, 5000) = 1 ([{fd=9, revents=POLLIN}]) <0.000500>\n");
        let v = parse_strace(&t);
        assert_eq!(v, json!([{"server": "192.0.2.53", "count": 2, "max_s": 5.0, "unfinished": 0}]));
    }

    #[test]
    fn nimble_log_maps_threads_to_pull_hosts() {
        let t = lines("[2026-01-01 00:00:00 P100-T101] [mpeg2tscamera3] E: media sequence reset for url='https://hls-a.example/live/index.m3u8' prev=5 new=3 current=3 chunks=5\n\
[2026-01-01 00:00:01 P100-T102] [mpeg2tscamera1] E: failed to resolve 'hls-b.example' h_errnop=-2\n\
[2026-01-01 00:00:01 P99-T98] [mpeg2tscamera0] E: media sequence reset for url='https://old.example/x.m3u8' prev=5 new=3 current=3 chunks=5\n");
        let v = parse_nimble_log(&t, Some("100"));
        assert_eq!(v["pid"], "100");
        assert_eq!(v["thread_hosts"], json!({"mpeg2tscamera3": ["hls-a.example"]}));
        assert_eq!(v["resolve_failures"], json!({"hls-b.example": 1}));
    }

    #[test]
    fn resolv_options() {
        let v = parse_resolv(&lines("nameserver 192.0.2.53\nnameserver 192.0.2.54 # ok\noptions timeout:1 attempts:2 rotate\n"));
        assert_eq!(v, json!({"nameservers": ["192.0.2.53", "192.0.2.54"], "timeout": 1, "attempts": 2}));
    }
}
