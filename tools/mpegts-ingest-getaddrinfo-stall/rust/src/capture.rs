// Wire-level capture via AF_PACKET (sees what the NIC delivered, before any UDP socket queue).

use crate::wire::Group;
use std::collections::HashMap;
use std::io;
use std::net::Ipv4Addr;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

const SOL_PACKET: libc::c_int = 263;
const PACKET_STATISTICS: libc::c_int = 6;
const PACKET_OUTGOING: u8 = 4;
const SO_TIMESTAMPNS: libc::c_int = 35;

pub struct Shared {
    pub groups: Vec<Arc<Mutex<Group>>>,
    pub index: HashMap<(u32, u16), usize>,
    pub other: Mutex<HashMap<(u32, u16), (u64, u64)>>,
    pub cap_packets: AtomicU64,
    pub cap_drops: AtomicU64,
    pub stop: AtomicBool,
}

#[repr(C)]
struct TpacketStats {
    tp_packets: u32,
    tp_drops: u32,
}

#[allow(dead_code)]
pub struct PacketSocket {
    fd: libc::c_int,
    pub iface: String,
    pub rcvbuf: i32,
}

impl Drop for PacketSocket {
    fn drop(&mut self) {
        unsafe { libc::close(self.fd) };
    }
}

fn bpf(code: u16, jt: u8, jf: u8, k: u32) -> libc::sock_filter {
    libc::sock_filter { code, jt, jf, k }
}

pub fn ifindex(name: &str) -> Option<i32> {
    let c = std::ffi::CString::new(name).ok()?;
    let idx = unsafe { libc::if_nametoindex(c.as_ptr()) };
    if idx == 0 { None } else { Some(idx as i32) }
}

/// Find interface name that owns the given IPv4 address.
pub fn iface_for_ip(ip: Ipv4Addr) -> Option<String> {
    unsafe {
        let mut head: *mut libc::ifaddrs = std::ptr::null_mut();
        if libc::getifaddrs(&mut head) != 0 {
            return None;
        }
        let mut cur = head;
        let mut found = None;
        while !cur.is_null() {
            let ifa = &*cur;
            if !ifa.ifa_addr.is_null() && (*ifa.ifa_addr).sa_family as i32 == libc::AF_INET {
                let sin = &*(ifa.ifa_addr as *const libc::sockaddr_in);
                let addr = Ipv4Addr::from(u32::from_be(sin.sin_addr.s_addr));
                if addr == ip {
                    found = Some(std::ffi::CStr::from_ptr(ifa.ifa_name).to_string_lossy().into_owned());
                    break;
                }
            }
            cur = ifa.ifa_next;
        }
        libc::freeifaddrs(head);
        found
    }
}

impl PacketSocket {
    pub fn open(iface: &str, rcvbuf: i32) -> io::Result<Self> {
        let idx = ifindex(iface).ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "interface not found"))?;
        let proto = (libc::ETH_P_ALL as u16).to_be();
        let fd = unsafe { libc::socket(libc::AF_PACKET, libc::SOCK_RAW, proto as libc::c_int) };
        if fd < 0 {
            return Err(io::Error::last_os_error());
        }
        let ps = PacketSocket { fd, iface: iface.to_string(), rcvbuf: 0 };

        // BPF: IPv4 && UDP && dst in 224.0.0.0/4
        let mut prog = [
            bpf(0x28, 0, 0, 12),          // ldh [12]
            bpf(0x15, 0, 6, 0x0800),      // jeq IPv4 else drop
            bpf(0x30, 0, 0, 23),          // ldb [23]
            bpf(0x15, 0, 4, 17),          // jeq UDP else drop
            bpf(0x20, 0, 0, 30),          // ld [30]
            bpf(0x54, 0, 0, 0xF000_0000), // and
            bpf(0x15, 0, 1, 0xE000_0000), // jeq multicast else drop
            bpf(0x06, 0, 0, 0x0004_0000), // ret accept
            bpf(0x06, 0, 0, 0),           // ret drop
        ];
        let fprog = libc::sock_fprog { len: prog.len() as u16, filter: prog.as_mut_ptr() };
        set_opt(fd, libc::SOL_SOCKET, libc::SO_ATTACH_FILTER, &fprog)?;

        // Large receive buffer (FORCE needs CAP_NET_ADMIN, fall back otherwise)
        if set_opt(fd, libc::SOL_SOCKET, libc::SO_RCVBUFFORCE, &rcvbuf).is_err() {
            let _ = set_opt(fd, libc::SOL_SOCKET, libc::SO_RCVBUF, &rcvbuf);
        }
        let one: libc::c_int = 1;
        set_opt(fd, libc::SOL_SOCKET, SO_TIMESTAMPNS, &one)?;
        let tv = libc::timeval { tv_sec: 0, tv_usec: 200_000 };
        set_opt(fd, libc::SOL_SOCKET, libc::SO_RCVTIMEO, &tv)?;

        let mut sll: libc::sockaddr_ll = unsafe { std::mem::zeroed() };
        sll.sll_family = libc::AF_PACKET as u16;
        sll.sll_protocol = proto;
        sll.sll_ifindex = idx;
        let r = unsafe {
            libc::bind(
                fd,
                &sll as *const _ as *const libc::sockaddr,
                std::mem::size_of::<libc::sockaddr_ll>() as u32,
            )
        };
        if r != 0 {
            return Err(io::Error::last_os_error());
        }
        let mut actual: libc::c_int = 0;
        let mut len = std::mem::size_of::<libc::c_int>() as libc::socklen_t;
        unsafe {
            libc::getsockopt(fd, libc::SOL_SOCKET, libc::SO_RCVBUF, &mut actual as *mut _ as *mut libc::c_void, &mut len)
        };
        // drain stats accumulated before the filter was attached
        let mut ps = ps;
        ps.rcvbuf = actual;
        let _ = ps.stats();
        Ok(ps)
    }

    /// Returns (packets, drops) since last call (kernel resets counters on read).
    pub fn stats(&self) -> (u64, u64) {
        let mut st = TpacketStats { tp_packets: 0, tp_drops: 0 };
        let mut len = std::mem::size_of::<TpacketStats>() as libc::socklen_t;
        let r = unsafe {
            libc::getsockopt(self.fd, SOL_PACKET, PACKET_STATISTICS, &mut st as *mut _ as *mut libc::c_void, &mut len)
        };
        if r != 0 { (0, 0) } else { (st.tp_packets as u64, st.tp_drops as u64) }
    }

    pub fn run(&self, shared: &Shared) {
        let mut buf = vec![0u8; 65_536];
        let mut ctrl = [0u64; 16];
        let mut last_stats = std::time::Instant::now();
        while !shared.stop.load(Ordering::Relaxed) {
            if last_stats.elapsed().as_millis() >= 500 {
                let (p, d) = self.stats();
                shared.cap_packets.fetch_add(p, Ordering::Relaxed);
                shared.cap_drops.fetch_add(d, Ordering::Relaxed);
                last_stats = std::time::Instant::now();
            }
            let mut sll: libc::sockaddr_ll = unsafe { std::mem::zeroed() };
            let mut iov = libc::iovec { iov_base: buf.as_mut_ptr() as *mut libc::c_void, iov_len: buf.len() };
            let mut msg: libc::msghdr = unsafe { std::mem::zeroed() };
            msg.msg_name = &mut sll as *mut _ as *mut libc::c_void;
            msg.msg_namelen = std::mem::size_of::<libc::sockaddr_ll>() as u32;
            msg.msg_iov = &mut iov;
            msg.msg_iovlen = 1;
            msg.msg_control = ctrl.as_mut_ptr() as *mut libc::c_void;
            msg.msg_controllen = std::mem::size_of_val(&ctrl) as _;
            let n = unsafe { libc::recvmsg(self.fd, &mut msg, libc::MSG_TRUNC) };
            if n < 0 {
                continue; // timeout or EINTR
            }
            if sll.sll_pkttype == PACKET_OUTGOING {
                continue;
            }
            let n = (n as usize).min(buf.len());
            let mut ts = 0u64;
            unsafe {
                let mut c = libc::CMSG_FIRSTHDR(&msg);
                while !c.is_null() {
                    if (*c).cmsg_level == libc::SOL_SOCKET && (*c).cmsg_type == SO_TIMESTAMPNS {
                        let t = std::ptr::read_unaligned(libc::CMSG_DATA(c) as *const libc::timespec);
                        ts = t.tv_sec as u64 * 1_000_000_000 + t.tv_nsec as u64;
                    }
                    c = libc::CMSG_NXTHDR(&msg, c);
                }
            }
            if ts == 0 {
                ts = crate::util::now_ns();
            }
            dispatch(shared, ts, &buf[..n]);
        }
        let (p, d) = self.stats();
        shared.cap_packets.fetch_add(p, Ordering::Relaxed);
        shared.cap_drops.fetch_add(d, Ordering::Relaxed);
    }
}

fn set_opt<T>(fd: libc::c_int, level: libc::c_int, name: libc::c_int, val: &T) -> io::Result<()> {
    let r = unsafe {
        libc::setsockopt(fd, level, name, val as *const T as *const libc::c_void, std::mem::size_of::<T>() as u32)
    };
    if r != 0 { Err(io::Error::last_os_error()) } else { Ok(()) }
}

fn dispatch(shared: &Shared, ts: u64, frame: &[u8]) {
    let mut off = 12;
    if frame.len() < 14 {
        return;
    }
    let mut et = u16::from_be_bytes([frame[12], frame[13]]);
    while et == 0x8100 || et == 0x88A8 {
        off += 4;
        if frame.len() < off + 2 {
            return;
        }
        et = u16::from_be_bytes([frame[off], frame[off + 1]]);
    }
    if et != 0x0800 {
        return;
    }
    let ip = off + 2;
    if frame.len() < ip + 20 {
        return;
    }
    let ihl = ((frame[ip] & 0x0F) as usize) * 4;
    if frame[ip + 9] != 17 || ihl < 20 || frame.len() < ip + ihl + 8 {
        return;
    }
    let total_len = u16::from_be_bytes([frame[ip + 2], frame[ip + 3]]) as usize;
    let frag_field = u16::from_be_bytes([frame[ip + 6], frame[ip + 7]]);
    let frag = (frag_field & 0x2000) != 0 || (frag_field & 0x1FFF) != 0;
    let ttl = frame[ip + 8];
    let src = u32::from_be_bytes([frame[ip + 12], frame[ip + 13], frame[ip + 14], frame[ip + 15]]);
    let dst = u32::from_be_bytes([frame[ip + 16], frame[ip + 17], frame[ip + 18], frame[ip + 19]]);
    let udp = ip + ihl;
    let dport = u16::from_be_bytes([frame[udp + 2], frame[udp + 3]]);
    let ulen = u16::from_be_bytes([frame[udp + 4], frame[udp + 5]]) as usize;
    let ip_end = (ip + total_len).min(frame.len());
    let pay_start = udp + 8;
    let pay_end = if ulen >= 8 { (udp + ulen).min(ip_end) } else { ip_end };
    if pay_end < pay_start {
        return;
    }
    let payload = &frame[pay_start..pay_end];
    match shared.index.get(&(dst, dport)) {
        Some(&i) => {
            if let Ok(mut g) = shared.groups[i].lock() {
                g.on_packet(ts, frame, payload, src, ttl, frag);
            }
        }
        None => {
            if let Ok(mut o) = shared.other.lock() {
                if o.len() < 256 || o.contains_key(&(dst, dport)) {
                    let e = o.entry((dst, dport)).or_insert((0, 0));
                    e.0 += 1;
                    e.1 += payload.len() as u64;
                }
            }
        }
    }
}

/// Keep IGMP membership for a group without receiving its data
/// (socket is bound to an ephemeral port, so no datagrams are queued on it).
pub fn join_only(group: Ipv4Addr, mip: Ipv4Addr) -> io::Result<libc::c_int> {
    let fd = unsafe { libc::socket(libc::AF_INET, libc::SOCK_DGRAM, 0) };
    if fd < 0 {
        return Err(io::Error::last_os_error());
    }
    let mreq = libc::ip_mreq {
        imr_multiaddr: libc::in_addr { s_addr: u32::from_ne_bytes(group.octets()) },
        imr_interface: libc::in_addr { s_addr: u32::from_ne_bytes(mip.octets()) },
    };
    if let Err(e) = set_opt(fd, libc::IPPROTO_IP, libc::IP_ADD_MEMBERSHIP, &mreq) {
        unsafe { libc::close(fd) };
        return Err(e);
    }
    Ok(fd)
}
