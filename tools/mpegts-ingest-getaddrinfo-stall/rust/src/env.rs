// One-shot environment snapshot: kernel, OS, sysctls, NIC, IGMP, Nimble package/config.

use std::fs;
use std::path::Path;
use std::process::Command;

fn run(dir: &Path, file: &str, cmd: &str) {
    let out = Command::new("sh").arg("-c").arg(cmd).output();
    let text = match out {
        Ok(o) => {
            let mut s = format!("$ {}\n", cmd);
            s.push_str(&String::from_utf8_lossy(&o.stdout));
            if !o.stderr.is_empty() {
                s.push_str("\n--- stderr ---\n");
                s.push_str(&String::from_utf8_lossy(&o.stderr));
            }
            s
        }
        Err(e) => format!("$ {}\nfailed: {}\n", cmd, e),
    };
    let _ = fs::write(dir.join(file), text);
}

fn copy(dir: &Path, file: &str, src: &str) {
    match fs::read(src) {
        Ok(b) => {
            let _ = fs::write(dir.join(file), b);
        }
        Err(e) => {
            let _ = fs::write(dir.join(file), format!("cannot read {}: {}\n", src, e));
        }
    }
}

/// Copy a config file, masking values on lines that look like credentials.
fn copy_masked(dir: &Path, file: &str, src: &str) {
    let text = match fs::read_to_string(src) {
        Ok(t) => t,
        Err(e) => {
            let _ = fs::write(dir.join(file), format!("cannot read {}: {}\n", src, e));
            return;
        }
    };
    let secret = ["pass", "key", "token", "secret", "auth", "login", "user", "uuid", "license", "cert"];
    let mut out = String::new();
    for line in text.lines() {
        let low = line.to_ascii_lowercase();
        if let Some(eq) = line.find('=') {
            let k = low[..eq].trim();
            if secret.iter().any(|s| k.contains(s)) || k == "id" || k.ends_with("_id") {
                out.push_str(&line[..eq + 1]);
                out.push_str(" ***masked***\n");
                continue;
            }
        }
        out.push_str(line);
        out.push('\n');
    }
    let _ = fs::write(dir.join(file), out);
}

fn sysctls(ifaces: &[String]) -> String {
    let mut keys: Vec<String> = [
        "net/core/rmem_max",
        "net/core/rmem_default",
        "net/core/wmem_max",
        "net/core/netdev_max_backlog",
        "net/core/netdev_budget",
        "net/core/busy_read",
        "net/core/busy_poll",
        "net/ipv4/udp_mem",
        "net/ipv4/udp_rmem_min",
        "net/ipv4/igmp_max_memberships",
        "net/ipv4/conf/all/rp_filter",
        "net/ipv4/conf/default/rp_filter",
        "net/ipv4/conf/all/force_igmp_version",
        "kernel/osrelease",
        "kernel/version",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
    for i in ifaces {
        keys.push(format!("net/ipv4/conf/{}/rp_filter", i));
        keys.push(format!("net/ipv4/conf/{}/force_igmp_version", i));
    }
    let mut s = String::new();
    for k in keys {
        let v = fs::read_to_string(format!("/proc/sys/{}", k)).unwrap_or_else(|_| "n/a".into());
        s.push_str(&format!("{} = {}\n", k.replace('/', "."), v.trim()));
    }
    s
}

pub fn snapshot(dir: &Path, ifaces: &[String], nimble_pid: Option<i32>, nimble_conf: &str) {
    let _ = fs::create_dir_all(dir);
    run(dir, "uname.txt", "uname -a");
    copy(dir, "os-release.txt", "/etc/os-release");
    run(dir, "uptime.txt", "uptime; echo; who -b 2>/dev/null; echo; last -x reboot 2>/dev/null | head -20");
    run(dir, "cpu_mem.txt", "nproc; grep -m1 'model name' /proc/cpuinfo; grep -E 'MemTotal|MemFree|MemAvailable' /proc/meminfo");
    let _ = fs::write(dir.join("sysctl.txt"), sysctls(ifaces));
    run(dir, "ip_addr.txt", "ip -d addr 2>/dev/null || ifconfig -a");
    run(dir, "ip_link_stats.txt", "ip -s -s -d link 2>/dev/null");
    run(dir, "ip_route.txt", "ip route 2>/dev/null; echo; ip maddr 2>/dev/null");
    copy(dir, "proc_net_igmp.txt", "/proc/net/igmp");
    copy(dir, "proc_net_dev_mcast.txt", "/proc/net/dev_mcast");
    copy(dir, "proc_net_udp_start.txt", "/proc/net/udp");
    copy(dir, "proc_net_snmp_start.txt", "/proc/net/snmp");
    copy(dir, "proc_net_softnet_start.txt", "/proc/net/softnet_stat");
    copy(dir, "proc_interrupts.txt", "/proc/interrupts");
    copy(dir, "proc_softirqs.txt", "/proc/softirqs");
    for i in ifaces {
        let n = crate::util::sanitize(i);
        run(dir, &format!("ethtool_{}.txt", n),
            &format!("for o in -i -g -k -c -l -S; do echo \"=== ethtool $o {0}\"; ethtool $o {0} 2>&1; done", i));
    }
    run(dir, "packages.txt",
        "dpkg-query -W -f='${Package} ${Version} ${Status}\\n' 'nimble*' 'linux-image*' 2>/dev/null | grep -v 'not-installed'");
    run(dir, "apt_history.txt",
        "zgrep -h -B1 -A4 -i -E 'linux-image|nimble' /var/log/apt/history.log* 2>/dev/null | tail -300");
    run(dir, "boot_dir.txt", "ls -lt /boot 2>/dev/null");
    run(dir, "dmesg_tail.txt", "dmesg 2>/dev/null | tail -300");
    copy_masked(dir, "nimble.conf.masked", nimble_conf);
    // resolver setup: glibc reads at most 3 nameservers; before glibc 2.26 a running
    // process keeps the resolv.conf it loaded at start
    copy(dir, "resolv.conf.txt", "/etc/resolv.conf");
    run(dir, "resolver.txt",
        "ls -l /etc/resolv.conf; echo; grep -E '^hosts' /etc/nsswitch.conf; echo; ldd --version 2>&1 | head -1");
    if let Some(pid) = nimble_pid {
        run(dir, "nimble_proc.txt", &format!(
            "cat /proc/{0}/cmdline | tr '\\0' ' '; echo; echo; cat /proc/{0}/status; echo; cat /proc/{0}/limits; echo; \
             ps -L -o tid,comm,psr,pri,ni,stat,pcpu -p {0} 2>/dev/null", pid));
    }
}
