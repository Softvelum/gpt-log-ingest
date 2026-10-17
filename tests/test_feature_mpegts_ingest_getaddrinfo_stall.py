"""mpegts-ingest-getaddrinfo-stall feature checks: repository layout and the Python
analyzer over synthetic fixtures (standard library only; nothing touches Nimble)."""

from pathlib import Path
import contextlib
import io
import json
import re
import subprocess
import sys
import tempfile
import unittest

from test_repository import local_links


ROOT = Path(__file__).resolve().parents[1]
SKILL = ROOT / ".agents/skills/mpegts-ingest-getaddrinfo-stall/SKILL.md"
WORKFLOW = ROOT / "workflows/mpegts-ingest-getaddrinfo-stall.md"
TOOL = ROOT / "tools/mpegts-ingest-getaddrinfo-stall"
sys.path.insert(0, str(TOOL / "python"))
sys.path.insert(0, str(TOOL / "tests"))

import check_expected  # noqa: E402
import ingest_stall  # noqa: E402


class IngestStallLayoutTests(unittest.TestCase):
    def test_skill_routes_to_its_workflow_and_tools(self):
        self.assertIn(WORKFLOW, set(local_links(SKILL)))
        for name in (
            "README.md", ".gitignore", "collect_dns_evidence.sh", "python/ingest_stall.py",
            "rust/Cargo.toml", "rust/Cargo.lock", "rust/src/main.rs", "rust/src/stall.rs",
            "tests/run_tests.sh", "tests/make_fixtures.py", "tests/expected.json",
            "tests/live_lab.sh", "testkit/fake_nimble.py", "testkit/dns_lab.py",
        ):
            with self.subTest(file=name):
                self.assertTrue((TOOL / name).is_file())

    def test_workflow_keeps_its_boundaries(self):
        text = " ".join(WORKFLOW.read_text().lower().split())
        for phrase in ("explicit request", "never change", "proposals", "redact", "strace"):
            with self.subTest(phrase=phrase):
                self.assertIn(phrase, text)

    def test_sources_contain_only_synthetic_addresses(self):
        # tool sources and fixtures may only use documentation, loopback,
        # multicast test and unspecified addresses
        allowed = re.compile(
            r"^(192\.0\.2\.|198\.51\.100\.|203\.0\.113\.|233\.252\.0\.|127\.|0\.0\.0\.0$|224\.0\.0\.0$)")
        ipv4 = re.compile(r"(?<![\w.])(\d{1,3}(?:\.\d{1,3}){3})(?=\.\d+\b|(?![\w.]))")
        for path in sorted(TOOL.rglob("*")):
            if "target" in path.parts or not path.is_file() or path.suffix not in (".py", ".rs", ".sh", ".md", ".json", ".sample"):
                continue
            for ip in ipv4.findall(path.read_text()):
                with self.subTest(file=str(path.relative_to(ROOT)), ip=ip):
                    self.assertRegex(ip, allowed)


class IngestStallAnalyzerTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.tmp = tempfile.TemporaryDirectory()
        cls.fixtures = Path(cls.tmp.name) / "fixtures"
        subprocess.run(
            [sys.executable, "-I", str(TOOL / "tests/make_fixtures.py"), str(cls.fixtures)],
            check=True, stdout=subprocess.DEVNULL,
        )

    @classmethod
    def tearDownClass(cls):
        cls.tmp.cleanup()

    def analyze(self, name, args):
        d = self.fixtures / name
        files = dict(zip(args[::2], (str(d / a) for a in args[1::2])))
        return ingest_stall.analyze(str(d), None, files.get("--dns"), files.get("--strace"),
                                    files.get("--nimble-log"))

    def test_fixtures_get_expected_verdicts(self):
        for name, exp in sorted(check_expected.expected().items()):
            with self.subTest(scenario=name):
                report = self.analyze(name, exp["args"])
                self.assertEqual(check_expected.check(report, exp), [])

    def test_report_is_json_serializable_and_lists_missing_inputs(self):
        report = self.analyze("blocked_no_dns_evidence", [])
        json.dumps(report)
        self.assertTrue(any("DNS check" in m for m in report["missing"]))
        self.assertEqual(report["evidence"], [])

    def test_udp_command_flags_full_queues_and_dns_sockets(self):
        out = io.StringIO()
        with contextlib.redirect_stdout(out):
            ingest_stall.main(["udp", str(self.fixtures / "blocked_resolver/env/proc_net_udp_end.txt"), "--json"])
        report = json.loads(out.getvalue())
        self.assertEqual(report["full_queue"], ["233.252.0.10:1234"])
        self.assertEqual(report["dns_sockets"], [{"server": "192.0.2.53", "count": 3}])

    def test_tcpdump_parser_handles_plain_and_verbose_forms(self):
        lines = [
            "12:00:00.000000 IP (tos 0x0, ttl 64, id 1, offset 0, flags [DF], proto UDP (17), length 60)",
            "    198.51.100.11.40000 > 192.0.2.53.53: [bad udp cksum 0x1 -> 0x2!] 100+ A? hls-a.example. (32)",
            "12:00:02.500000 IP 192.0.2.53.53 > 198.51.100.11.40000: 100 ServFail 0/0/0 (32)",
            "12:00:03.000000 IP 198.51.100.11.40001 > 192.0.2.54.53: 101+ A? hls-a.example. (32)",
        ]
        servers = {s["server"]: s for s in ingest_stall.parse_dns_text(lines)["servers"]}
        self.assertEqual(servers["192.0.2.53"]["rcodes"], {"ServFail": 1})
        self.assertEqual(servers["192.0.2.53"]["median_ms"], 2500.0)
        self.assertEqual(servers["192.0.2.54"]["unanswered"], 1)


if __name__ == "__main__":
    unittest.main()
