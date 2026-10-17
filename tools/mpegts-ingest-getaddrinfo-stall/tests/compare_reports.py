#!/usr/bin/env python3
"""Compare two analyzer JSON reports (Python vs Rust): exact structure and values,
floats within 0.011. Exit 0 when they match, 1 with a list of differences."""
import json
import sys


def diff(a, b, path, out):
    if isinstance(a, bool) or isinstance(b, bool):
        if a is not b:
            out.append("%s: %r != %r" % (path, a, b))
    elif isinstance(a, (int, float)) and isinstance(b, (int, float)):
        if isinstance(a, float) or isinstance(b, float):
            if abs(a - b) > 0.011:
                out.append("%s: %r != %r" % (path, a, b))
        elif a != b:
            out.append("%s: %r != %r" % (path, a, b))
    elif isinstance(a, dict) and isinstance(b, dict):
        for k in sorted(set(a) | set(b)):
            if k not in a or k not in b:
                out.append("%s.%s: present only in %s" % (path, k, "first" if k in a else "second"))
            else:
                diff(a[k], b[k], "%s.%s" % (path, k), out)
    elif isinstance(a, list) and isinstance(b, list):
        if len(a) != len(b):
            out.append("%s: length %d != %d" % (path, len(a), len(b)))
        for i, (x, y) in enumerate(zip(a, b)):
            diff(x, y, "%s[%d]" % (path, i), out)
    elif a != b:
        out.append("%s: %r != %r" % (path, a, b))


def main():
    if len(sys.argv) != 3:
        sys.exit("usage: compare_reports.py A.json B.json")
    with open(sys.argv[1]) as f:
        a = json.load(f)
    with open(sys.argv[2]) as f:
        b = json.load(f)
    out = []
    diff(a, b, "$", out)
    for line in out[:50]:
        print(line)
    if out:
        print("%d difference(s)" % len(out))
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
