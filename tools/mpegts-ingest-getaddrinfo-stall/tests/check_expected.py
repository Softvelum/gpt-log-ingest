#!/usr/bin/env python3
"""Run one analyzer implementation over the synthetic fixtures and check verdicts.

  check_expected.py FIXTURES OUTDIR -- CMD [ARGS...]

CMD is the analyzer, for example `python3 -I ../python/ingest_stall.py` or
`../rust/target/release/nimdiag`. Each report is saved as OUTDIR/<scenario>.json
for the Python/Rust parity comparison.
"""
import json
import os
import subprocess
import sys

HERE = os.path.dirname(os.path.abspath(__file__))


def expected():
    with open(os.path.join(HERE, "expected.json")) as f:
        return json.load(f)


def check(report, exp):
    """Return a list of mismatches between a report and its expectation."""
    bad = []

    def eq(what, got, want):
        if got != want:
            bad.append("%s: got %r, want %r" % (what, got, want))

    eq("verdict", report["verdict"], exp["verdict"])
    eq("affected_streams", report["affected_streams"], exp["affected_streams"])
    first = report["streams"][0] if report["streams"] else {}
    if "reader" in exp:
        eq("reader", (first.get("reader") or {}).get("name"), exp["reader"])
    if "blocked" in exp:
        eq("blocked", sorted(b["name"] for b in report["threads"]["blocked"]), exp["blocked"])
    if "source" in exp:
        eq("source", report["threads"]["source"], exp["source"])
    if "period_s" in exp:
        eq("period_s", first.get("period_s"), exp["period_s"])
    if "bad_servers" in exp:
        eq("bad_servers", [s["server"] for s in report["dns"]["servers"] if s["bad"]], exp["bad_servers"])
    if "pull_hosts" in exp:
        eq("pull_hosts", report["threads"]["pull_hosts"], exp["pull_hosts"])
    if "reason_contains" in exp:
        text = " ".join(r for s in report["streams"] for r in s["reasons"])
        if exp["reason_contains"] not in text:
            bad.append("reasons do not mention %r" % exp["reason_contains"])
    return bad


def run(cmd, fixtures, name, exp):
    d = os.path.join(fixtures, name)
    args = []
    for a in exp["args"]:
        args.append(os.path.join(d, a) if not a.startswith("-") else a)
    p = subprocess.run(cmd + ["analyze", d] + args + ["--json"], stdout=subprocess.PIPE,
                       stderr=subprocess.PIPE, universal_newlines=True)
    if p.returncode != 0:
        raise RuntimeError("%s failed (%d): %s" % (name, p.returncode, p.stderr.strip()))
    return json.loads(p.stdout)


def main():
    if "--" not in sys.argv or sys.argv.index("--") != 3:
        sys.exit(__doc__)
    fixtures, out = sys.argv[1], sys.argv[2]
    cmd = sys.argv[4:]
    os.makedirs(out, exist_ok=True)
    failed = 0
    for name, exp in sorted(expected().items()):
        try:
            report = run(cmd, fixtures, name, exp)
        except RuntimeError as e:
            print("FAIL %s: %s" % (name, e))
            failed += 1
            continue
        with open(os.path.join(out, name + ".json"), "w") as f:
            json.dump(report, f, indent=2, sort_keys=True)
        bad = check(report, exp)
        print("%s %-26s %s" % ("FAIL" if bad else "ok  ", name, report["verdict"]))
        for b in bad:
            print("     " + b)
        failed += bool(bad)
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main())
