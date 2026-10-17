#!/bin/bash
# Live lab reproduction of the blocked-resolver ingest stall, fully isolated.
#
#   sudo tests/live_lab.sh [OUTDIR]
#
# Runs in new network, PID and mount namespaces (unshare): the real host network,
# any real Nimble process and the host resolver configuration are invisible and
# untouched. Inside, a fake `nimble` (testkit/fake_nimble.py) reads two synthetic
# multicast inputs and resolves pull host names with the real glibc getaddrinfo()
# on one of the input threads. resolv.conf (bind-mounted only inside the namespace)
# lists a silent nameserver first. nimdiag collect records the run, the DNS
# evidence script captures DNS and strace, and both analyzers must report
# "confirmed" for CHANNEL_A only. Needs root, unshare, ip, python3 >= 3.8,
# tcpdump and strace; takes about 90 s.

set -eu
HERE=$(cd "$(dirname "$0")/.." && pwd)
BIN="$HERE/rust/target/x86_64-unknown-linux-gnu/release/nimdiag"
[ "$(id -u)" = 0 ] || { echo "live_lab.sh needs root (namespaces, AF_PACKET, strace)" >&2; exit 2; }
for c in unshare ip python3 tcpdump strace; do
    command -v "$c" >/dev/null || { echo "missing: $c" >&2; exit 2; }
done
[ -x "$BIN" ] || (cd "$HERE/rust" && cargo build --release --locked)
OUT=${1:-$(mktemp -d)}
mkdir -p "$OUT"
echo "lab output: $OUT"

unshare --mount --net --pid --fork --mount-proc --propagation private \
    env HERE="$HERE" BIN="$BIN" OUT="$OUT" bash -eu -c '
K="$HERE/testkit"
ip link set lo up
ip route add 224.0.0.0/4 dev lo
printf "nameserver 127.0.0.77\nnameserver 127.0.0.78\noptions timeout:3 attempts:1\n" > "$OUT/resolv.conf"
printf "hosts: files dns\n" > "$OUT/nsswitch.conf"
mount --bind "$OUT/resolv.conf" /etc/resolv.conf
mount --bind "$OUT/nsswitch.conf" /etc/nsswitch.conf

python3 -I "$K/dns_lab.py" --silent 127.0.0.77:53 --good 127.0.0.78:53 &
sleep 0.5
python3 -I "$K/fake_nimble.py" --mip 127.0.0.1 --pull-period 6 \
    --input 233.252.0.10:1234@0 --pull hls-a.example@0 \
    --input 233.252.0.11:1234@1 --pull hls-b.example@2 --log "$OUT/nimble.log" &
FPID=$!
sleep 0.5
python3 -I "$K/ts_sender.py" 233.252.0.10 1234 --mbps 10 &
python3 -I "$K/ts_sender.py" 233.252.0.11 1234 --mbps 4 &

"$BIN" collect --config "$K/streams_test.json" --pid "$FPID" --out "$OUT" --duration 70 \
    --cooldown 10 --pre 6 --post 2 --ring-mb 8 --no-tar --console 10 \
    --nimble-conf "$K/nimble.conf.sample" > "$OUT/collect.log" 2>&1 &
NPID=$!
sleep 25
"$HERE/collect_dns_evidence.sh" -p "$FPID" -H hls-a.example -s 20 -i lo --strace "$OUT/dns" > /dev/null
wait "$NPID"
'

RUN=$(ls -d "$OUT"/nimdiag_* | head -1)
ARGS=("$RUN" --dns "$OUT/dns/dns.txt" --strace "$OUT/dns/strace.txt" --nimble-log "$OUT/nimble.log" --json)
python3 -I "$HERE/python/ingest_stall.py" analyze "${ARGS[@]}" > "$OUT/report.py.json"
"$BIN" analyze "${ARGS[@]}" > "$OUT/report.rs.json"

fail=0
python3 -I - "$OUT" "$RUN" <<'EOF' || fail=1
import json, os, sys
out, run = sys.argv[1], sys.argv[2]
r = json.load(open(os.path.join(out, "report.py.json")))
errors = []
if r["verdict"] != "confirmed":
    errors.append("verdict %s" % r["verdict"])
if r["affected_streams"] != ["CHANNEL_A"]:
    errors.append("affected %s" % r["affected_streams"])
reader = (r["streams"][0].get("reader") or {}).get("name") if r["streams"] else None
if reader != "mpeg2tscamera0":
    errors.append("reader %s" % reader)
blocked = sorted(b["name"] for b in r["threads"]["blocked"])
if "mpeg2tscamera0" not in blocked or "mpeg2tscamera1" in blocked:
    errors.append("blocked %s" % blocked)
if not any(s["bad"] and s["server"] == "127.0.0.77" for s in r["dns"]["servers"]):
    errors.append("silent nameserver not flagged: %s" % r["dns"]["servers"])
if not r["dns"]["strace_blocking"]:
    errors.append("strace found no blocking resolver poll")
conf = open(os.path.join(run, "env", "nimble.conf.masked")).read()
if "00000000-0000-4000" in conf or "synthetic-not-a-secret" in conf:
    errors.append("nimble.conf masking failed")
summary = open(os.path.join(run, "summary.txt")).read()
if "verdict: confirmed" not in summary:
    errors.append("collect summary has no confirmed stall check")
for e in errors:
    print("FAIL " + e)
print("live lab: %s (reader %s, blocked %s)" % ("FAIL" if errors else "ok", reader, ", ".join(blocked)))
sys.exit(1 if errors else 0)
EOF
python3 -I "$HERE/tests/compare_reports.py" "$OUT/report.py.json" "$OUT/report.rs.json" || { echo "FAIL Python/Rust parity"; fail=1; }
exit $fail
