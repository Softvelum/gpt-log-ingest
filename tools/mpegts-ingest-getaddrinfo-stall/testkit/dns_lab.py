#!/usr/bin/env python3
"""Two local DNS servers for the lab reproduction.

--silent ADDR:PORT   reads queries and never answers (a broken first nameserver)
--good ADDR:PORT     answers every A query with --answer (default 192.0.2.10)

Use only inside an isolated network namespace or on loopback addresses.
"""
import argparse
import socket
import struct
import threading


def answer(query, ip):
    if len(query) < 12:
        return None
    qid = query[:2]
    i = 12
    while i < len(query) and query[i] != 0:
        i += query[i] + 1
    q = query[12:i + 5]
    hdr = qid + struct.pack("!HHHHH", 0x8180, 1, 1, 0, 0)
    rr = b"\xc0\x0c" + struct.pack("!HHIH", 1, 1, 60, 4) + socket.inet_aton(ip)
    return hdr + q + rr


def serve(addr, reply_ip):
    host, _, port = addr.rpartition(":")
    s = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    s.bind((host, int(port)))
    while True:
        data, peer = s.recvfrom(4096)
        if reply_ip:
            r = answer(data, reply_ip)
            if r:
                s.sendto(r, peer)


def main():
    ap = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    ap.add_argument("--silent", action="append", default=[])
    ap.add_argument("--good", action="append", default=[])
    ap.add_argument("--answer", default="192.0.2.10")
    a = ap.parse_args()
    ts = [threading.Thread(target=serve, args=(x, None), daemon=True) for x in a.silent]
    ts += [threading.Thread(target=serve, args=(x, a.answer), daemon=True) for x in a.good]
    for t in ts:
        t.start()
    for t in ts:
        t.join()


if __name__ == "__main__":
    main()
