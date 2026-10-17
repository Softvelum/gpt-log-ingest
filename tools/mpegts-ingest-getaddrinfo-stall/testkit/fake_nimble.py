#!/usr/bin/env python3
"""Fake 'nimble' process for nimdiag tests.

Each ingest thread is named mpeg2tscameraN and runs an epoll loop over the UDP
multicast inputs assigned to it. A thread that also has pull inputs resolves
their host names with the real, blocking glibc getaddrinfo() every --pull-period
seconds, inside the same loop - the pattern this tool diagnoses. With a slow
first nameserver the thread sits in poll() and its UDP inputs are not read.

  fake_nimble.py --mip 127.0.0.1 \\
      --input 233.252.0.10:1234@0 --pull hls-a.example@0 \\
      --input 233.252.0.11:1234@1 --pull hls-b.example@2 --log nimble-test.log

--input GROUP:PORT@THREAD[:slow]   ':slow' reads one datagram per 100 ms instead
--pull HOST@THREAD                 resolve HOST periodically on that thread
--log FILE                         write Nimble-style lines that map threads to URLs
"""
import argparse
import ctypes
import os
import select
import socket
import threading
import time

libc = ctypes.CDLL(None, use_errno=True)
PR_SET_NAME = 15
SO_RCVBUFFORCE = 33
LOG_LOCK = threading.Lock()


def set_name(name):
    libc.prctl(PR_SET_NAME, ctypes.c_char_p(name.encode()), 0, 0, 0)


def log_line(path, thread, msg):
    if not path:
        return
    t = time.strftime("%Y-%m-%d %H:%M:%S", time.gmtime())
    line = "[%s P%d-T%d] [%s] E: %s\n" % (t, os.getpid(), threading.get_native_id(), thread, msg)
    with LOG_LOCK, open(path, "a") as f:
        f.write(line)


def camera(idx, inputs, pulls, args):
    name = "mpeg2tscamera%d" % idx
    set_name(name)
    ep = select.epoll()
    socks = {}
    slow = set()
    for group, port, is_slow in inputs:
        s = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
        s.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
        try:
            s.setsockopt(socket.SOL_SOCKET, SO_RCVBUFFORCE, args.rcvbuf)
        except OSError:
            s.setsockopt(socket.SOL_SOCKET, socket.SO_RCVBUF, args.rcvbuf)
        s.bind((group, port))
        mreq = socket.inet_aton(group) + socket.inet_aton(args.mip)
        s.setsockopt(socket.IPPROTO_IP, socket.IP_ADD_MEMBERSHIP, mreq)
        s.setblocking(False)
        socks[s.fileno()] = s
        if is_slow:
            slow.add(s.fileno())
        ep.register(s.fileno(), select.EPOLLIN)
    seq = 1000
    next_pull = time.monotonic() + args.pull_offset * idx
    while True:
        now = time.monotonic()
        if pulls and now >= next_pull:
            for host in pulls:
                try:
                    socket.getaddrinfo(host, 443, socket.AF_INET, socket.SOCK_STREAM)
                except socket.gaierror:
                    log_line(args.log, name, "failed to resolve '%s' h_errnop=-2" % host)
                seq += 1
                log_line(args.log, name, "media sequence reset for url='https://%s/live/index.m3u8' "
                                         "prev=%d new=%d current=%d chunks=5" % (host, seq, seq - 2, seq - 2))
            next_pull = time.monotonic() + args.pull_period
        if not socks:
            time.sleep(0.1)
            continue
        for fd, _ in ep.poll(0.1):
            s = socks[fd]
            try:
                if fd in slow:
                    s.recv(65536)
                    time.sleep(0.1)
                    continue
                while True:
                    s.recv(65536)
            except BlockingIOError:
                pass


def main():
    ap = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    ap.add_argument("--mip", default="127.0.0.1")
    ap.add_argument("--input", action="append", default=[])
    ap.add_argument("--pull", action="append", default=[])
    ap.add_argument("--pull-period", type=float, default=6.0)
    ap.add_argument("--pull-offset", type=float, default=0.3, help="stagger of pull threads, s")
    ap.add_argument("--rcvbuf", type=int, default=600000, help="requested SO_RCVBUF (kernel doubles it)")
    ap.add_argument("--log")
    args = ap.parse_args()
    threads = {}
    for spec in args.input:
        addr, _, rest = spec.partition("@")
        idx, _, flag = rest.partition(":")
        group, _, port = addr.partition(":")
        threads.setdefault(int(idx), ([], []))[0].append((group, int(port), flag == "slow"))
    for spec in args.pull:
        host, _, idx = spec.partition("@")
        threads.setdefault(int(idx), ([], []))[1].append(host)
    set_name("nimble")
    for idx in sorted(threads):
        inputs, pulls = threads[idx]
        threading.Thread(target=camera, args=(idx, inputs, pulls, args), daemon=True).start()
    while True:
        time.sleep(1)


if __name__ == "__main__":
    main()
