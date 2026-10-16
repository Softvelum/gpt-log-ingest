#!/bin/bash
# Offline tests for tools/mpegts-ingest-getaddrinfo-stall.
#
#   tests/run_tests.sh            Rust unit tests, synthetic fixtures through both
#                                 analyzers, expected verdicts, Python/Rust parity
#   LIVE=1 tests/run_tests.sh     also run the isolated live lab (root, ~90 s)
#
# The offline part needs no privileges and no network. Fixtures are generated into
# a temporary directory and removed afterwards.

set -u
HERE=$(cd "$(dirname "$0")/.." && pwd)
TMP=$(mktemp -d)
trap 'rm -rf "$TMP"' EXIT
fail=0
step() { printf '\n== %s\n' "$*"; }

step "cargo test"
(cd "$HERE/rust" && cargo test --locked --release 2>&1 | grep -E "^test |test result|error") || fail=1
step "cargo build --release"
(cd "$HERE/rust" && cargo build --locked --release 2>&1 | grep -E "^(warning|error)") && fail=1
BIN="$HERE/rust/target/x86_64-unknown-linux-gnu/release/nimdiag"
[ -x "$BIN" ] || { echo "nimdiag binary missing"; exit 1; }

step "synthetic fixtures"
python3 -I "$HERE/tests/make_fixtures.py" "$TMP/fixtures" || exit 1

step "Python analyzer: expected verdicts"
python3 -I "$HERE/tests/check_expected.py" "$TMP/fixtures" "$TMP/py" -- python3 -I "$HERE/python/ingest_stall.py" || fail=1
step "Rust analyzer: expected verdicts"
python3 -I "$HERE/tests/check_expected.py" "$TMP/fixtures" "$TMP/rs" -- "$BIN" || fail=1

step "Python/Rust parity (JSON and text)"
for f in "$TMP"/py/*.json; do
    n=$(basename "$f" .json)
    python3 -I "$HERE/tests/compare_reports.py" "$f" "$TMP/rs/$n.json" > "$TMP/diff.txt" \
        || { echo "FAIL json $n"; cat "$TMP/diff.txt"; fail=1; continue; }
    args=()
    [ -f "$TMP/fixtures/$n/dns.txt" ] && args=(--dns "$TMP/fixtures/$n/dns.txt" --strace "$TMP/fixtures/$n/strace.txt" --nimble-log "$TMP/fixtures/$n/nimble.log")
    if diff <(python3 -I "$HERE/python/ingest_stall.py" analyze "$TMP/fixtures/$n" "${args[@]+"${args[@]}"}") \
            <("$BIN" analyze "$TMP/fixtures/$n" "${args[@]+"${args[@]}"}") > "$TMP/diff.txt"; then
        echo "ok   $n"
    else
        echo "FAIL text $n"; cat "$TMP/diff.txt"; fail=1
    fi
done
for snap in proc_net_udp_start proc_net_udp_end; do
    f="$TMP/fixtures/blocked_resolver/env/$snap.txt"
    diff <(python3 -I "$HERE/python/ingest_stall.py" udp "$f" --json) <("$BIN" udp "$f" --json) > /dev/null \
        && echo "ok   udp $snap" || { echo "FAIL udp $snap"; fail=1; }
done

if [ "${LIVE:-0}" = 1 ]; then
    step "live lab (isolated namespaces)"
    "$HERE/tests/live_lab.sh" "$TMP/lab" || fail=1
else
    printf '\nlive lab not run (set LIVE=1, needs root)\n'
fi

printf '\n%s\n' "$([ $fail = 0 ] && echo "ALL TESTS PASSED$([ "${LIVE:-0}" = 1 ] || echo " (offline only)")" || echo "TESTS FAILED")"
exit $fail
