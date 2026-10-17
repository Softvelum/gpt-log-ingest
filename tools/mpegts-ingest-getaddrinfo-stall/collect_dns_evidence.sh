#!/bin/bash
# Collect DNS evidence for the blocked-resolver ingest stall on a Nimble host.
#
#   collect_dns_evidence.sh [-p PID] [-H HOST]... [-s SECONDS] [-i IFACE] [--strace] OUTDIR
#
# Writes OUTDIR/dns.txt: /etc/resolv.conf, `time getent ahosts HOST` for each -H,
# and a tcpdump of port 53 for SECONDS (default 30) on IFACE (default any).
# With --strace it also finds Nimble threads that sit in poll()/ppoll() and traces
# only those thread ids for SECONDS into OUTDIR/strace.txt. strace attaches to the
# running process, so use it only when the operator agreed to that.
#
# Nothing is changed on the host. Run as root (tcpdump, /proc/<pid>/task, strace).
# The output contains host names and addresses: keep it local or redact it.
# Analyze with: nimdiag analyze DIR --dns OUTDIR/dns.txt [--strace OUTDIR/strace.txt]

set -u
PID="" SECONDS_RUN=30 IFACE=any STRACE=0 OUT=""
HOSTS=()
while [ $# -gt 0 ]; do
    case "$1" in
        -p) PID="$2"; shift 2 ;;
        -H) HOSTS+=("$2"); shift 2 ;;
        -s) SECONDS_RUN="$2"; shift 2 ;;
        -i) IFACE="$2"; shift 2 ;;
        --strace) STRACE=1; shift ;;
        -h|--help) sed -n '2,15p' "$0" | sed 's/^# \{0,1\}//'; exit 0 ;;
        -*) echo "unknown option: $1" >&2; exit 2 ;;
        *) OUT="$1"; shift ;;
    esac
done
[ -n "$OUT" ] || { echo "usage: $0 [-p PID] [-H HOST]... [-s SECONDS] [-i IFACE] [--strace] OUTDIR" >&2; exit 2; }
[ -n "$PID" ] || PID=$(pgrep -x nimble | head -1)
mkdir -p "$OUT" || exit 1
DNS="$OUT/dns.txt"

{
    echo "\$ cat /etc/resolv.conf"
    cat /etc/resolv.conf
    echo
    TIMEFORMAT=$'real\t%3lR'
    for h in "${HOSTS[@]+"${HOSTS[@]}"}"; do
        echo "\$ time getent ahosts $h"
        { time getent ahosts "$h" >/dev/null; } 2>&1
        echo
    done
    echo "\$ timeout $SECONDS_RUN tcpdump -i $IFACE -nn -vv -l port 53"
} > "$DNS"

if command -v tcpdump >/dev/null; then
    timeout "$SECONDS_RUN" tcpdump -i "$IFACE" -nn -vv -l port 53 >> "$DNS" 2>/dev/null &
    TCPDUMP=$!
else
    echo "tcpdump not installed: capture skipped" >> "$DNS"
    TCPDUMP=""
fi

if [ "$STRACE" = 1 ]; then
    ST="$OUT/strace.txt"
    if [ -z "$PID" ] || [ ! -d "/proc/$PID/task" ]; then
        echo "nimble process not found (use -p PID): strace skipped" > "$ST"
    elif ! command -v strace >/dev/null; then
        echo "strace not installed: skipped" > "$ST"
    else
        # sample for ~5 s: threads whose current syscall is poll (7) or ppoll (271)
        declare -A SEEN=()
        for _ in 1 2 3 4 5 6 7 8 9 10; do
            for t in /proc/"$PID"/task/*; do
                sc=$(cut -d' ' -f1 "$t/syscall" 2>/dev/null)
                if [ "$sc" = 7 ] || [ "$sc" = 271 ]; then
                    SEEN[${t##*/}]=$(cat "$t/comm" 2>/dev/null)
                fi
            done
            sleep 0.5
        done
        {
            echo "# threads seen in poll(): ${!SEEN[*]}"
            for t in "${!SEEN[@]}"; do echo "#   $t ${SEEN[$t]}"; done
        } > "$ST"
        if [ ${#SEEN[@]} -eq 0 ]; then
            echo "# no thread was in poll() while sampling: strace skipped" >> "$ST"
        else
            ARGS=()
            for t in "${!SEEN[@]}"; do ARGS+=(-p "$t"); done
            # trace only those thread ids: -f on the whole process would slow every thread
            timeout "$SECONDS_RUN" strace -tt -T -e trace=connect,sendto,sendmmsg,recvfrom,poll,ppoll \
                "${ARGS[@]}" 2>&1 | grep -v EAGAIN >> "$ST"
        fi
    fi
fi

[ -n "$TCPDUMP" ] && wait "$TCPDUMP"
echo "written: $DNS${STRACE:+ }$( [ "$STRACE" = 1 ] && echo "$OUT/strace.txt")"
