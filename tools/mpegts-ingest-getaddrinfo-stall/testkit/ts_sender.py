#!/usr/bin/env python3
"""Synthetic MPEG-TS multicast sender for nimdiag testing.

Examples:
  ts_sender.py 233.252.0.10 1234 --mbps 4
  ts_sender.py 233.252.0.11 1234 --mbps 4 --burst 40 --cc-err-every 500
  ts_sender.py 233.252.0.12 1234 --mbps 2 --rtp
"""
import argparse
import socket
import struct
import time

PID_VIDEO = 0x100
PCR_INTERVAL_S = 0.04


def ts_packet(pid, cc, pcr=None):
    if pcr is not None:
        base, ext = divmod(pcr, 300)
        af = bytes([
            7, 0x10,
            (base >> 25) & 0xFF, (base >> 17) & 0xFF, (base >> 9) & 0xFF, (base >> 1) & 0xFF,
            ((base & 1) << 7) | 0x7E | ((ext >> 8) & 1), ext & 0xFF,
        ])
        hdr = bytes([0x47, (pid >> 8) & 0x1F, pid & 0xFF, 0x30 | (cc & 0x0F)])
        return hdr + af + b"\xff" * (188 - 4 - len(af))
    hdr = bytes([0x47, (pid >> 8) & 0x1F, pid & 0xFF, 0x10 | (cc & 0x0F)])
    return hdr + b"\xff" * 184


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("group")
    ap.add_argument("port", type=int)
    ap.add_argument("--mbps", type=float, default=4.0)
    ap.add_argument("--iface", default="127.0.0.1")
    ap.add_argument("--burst", type=int, default=0, help="send datagrams in bursts of N back-to-back")
    ap.add_argument("--cc-err-every", type=int, default=0, help="skip one CC value every N TS packets")
    ap.add_argument("--rtp", action="store_true")
    ap.add_argument("--duration", type=float, default=3600)
    a = ap.parse_args()

    s = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    s.setsockopt(socket.IPPROTO_IP, socket.IP_MULTICAST_IF, socket.inet_aton(a.iface))
    s.setsockopt(socket.IPPROTO_IP, socket.IP_MULTICAST_TTL, 4)
    s.setsockopt(socket.IPPROTO_IP, socket.IP_MULTICAST_LOOP, 1)

    dgram_bits = 7 * 188 * 8
    dgram_interval = dgram_bits / (a.mbps * 1e6)
    pkt_interval = dgram_interval / 7
    cc = 0
    ts_count = 0
    rtp_seq = 0
    next_pcr_at = 0.0
    start = time.monotonic()
    sent = 0
    group = max(1, a.burst)
    while time.monotonic() - start < a.duration:
        for _ in range(group):
            payload = bytearray()
            for _ in range(7):
                vt = ts_count * pkt_interval
                pcr = None
                if vt >= next_pcr_at:
                    pcr = int(vt * 27_000_000)
                    next_pcr_at += PCR_INTERVAL_S
                ts_count += 1
                if a.cc_err_every and ts_count % a.cc_err_every == 0:
                    cc = (cc + 1) & 0x0F
                payload += ts_packet(PID_VIDEO, cc, pcr)
                cc = (cc + 1) & 0x0F
            if a.rtp:
                hdr = struct.pack("!BBHII", 0x80, 33, rtp_seq & 0xFFFF, int(sent * dgram_interval * 90000) & 0xFFFFFFFF, 0x1234)
                payload = hdr + payload
                rtp_seq += 1
            s.sendto(bytes(payload), (a.group, a.port))
            sent += 1
        target = start + sent * dgram_interval
        delay = target - time.monotonic()
        if delay > 0:
            time.sleep(delay)


if __name__ == "__main__":
    main()
