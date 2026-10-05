"""Behavior checks for the Python 3 incremental watcher."""

import shutil
import subprocess
import tempfile
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]


class PythonWatcherTests(unittest.TestCase):
    def test_exact_levels_grouping_status_and_stage_timings(self):
        with tempfile.TemporaryDirectory(prefix="nimble-python-watch-") as directory:
            log_dir = Path(directory)
            shutil.copy(ROOT / "tests/fixtures/nimble-synthetic.log", log_dir / "nimble.log")
            result = subprocess.run(
                [
                    "python3", "scripts/nimble_log_watch.py", "--dir", str(log_dir),
                    "--include-existing", "--levels", "E,V,D,I",
                    "--duration-seconds", "1", "--color", "never",
                ],
                cwd=ROOT, capture_output=True, text=True, timeout=10,
            )
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn("interval_seconds=10", result.stdout)
        self.assertIn("[WATCH][STATE] status=waiting", result.stdout)
        self.assertIn("[WATCH][PERF]", result.stdout)
        for stage in ("read_us=", "parse_us=", "group_us=", "catalog_us=", "output_us="):
            self.assertIn(stage, result.stdout)
        self.assertIn("message level=V", result.stdout)
        self.assertIn("message level=D", result.stdout)
        self.assertIn("message level=E", result.stdout)
        self.assertIn("message level=I", result.stdout)
        self.assertIn("count=2", result.stdout)
        self.assertNotIn("\x1b[", result.stdout)

    def test_metrics_only_suppresses_message_content(self):
        with tempfile.TemporaryDirectory(prefix="nimble-python-metrics-") as directory:
            log_dir = Path(directory)
            shutil.copy(ROOT / "tests/fixtures/nimble-synthetic.log", log_dir / "nimble.log")
            result = subprocess.run(
                [
                    "python3", "scripts/nimble_log_watch.py", "--dir", str(log_dir),
                    "--include-existing", "--levels", "E,V,D,I", "--interval-seconds", "1",
                    "--duration-seconds", "1", "--color", "never", "--metrics-only",
                ],
                cwd=ROOT, capture_output=True, text=True, timeout=10,
            )
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn("[WATCH][PERF]", result.stdout)
        self.assertNotIn("message level=", result.stdout)
        self.assertNotIn("8.8.8.8", result.stdout)

    def test_unselected_log_levels_do_not_emit_batches(self):
        with tempfile.TemporaryDirectory(prefix="nimble-python-filter-") as directory:
            log_dir = Path(directory)
            (log_dir / "nimble.log").write_text(
                "[2026-10-05 10:00:00 P1-T2] [test] I: ignore this info record\n",
                encoding="utf-8",
            )
            result = subprocess.run(
                [
                    "python3", "scripts/nimble_log_watch.py", "--dir", str(log_dir),
                    "--levels", "E", "--duration-seconds", "1", "--color", "never",
                ],
                cwd=ROOT, capture_output=True, text=True, timeout=10,
            )
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn("status=waiting_for_next_poll", result.stdout)
        self.assertNotIn("[WATCH][BATCH]", result.stdout)
        self.assertNotIn("ignore this info record", result.stdout)

    def test_large_lag_resumes_at_next_complete_nimble_record(self):
        with tempfile.TemporaryDirectory(prefix="nimble-python-lag-") as directory:
            log_dir = Path(directory)
            (log_dir / "nimble.log").write_bytes(
                b"[2026-10-05 10:00:00 P1-T2] [test] E: old record\n"
                + b"x" * 33000
                + b"\nnot a timestamp record\n"
                + b"[2026-10-05 10:00:01 P1-T2] [test] E: recovered record\n"
            )
            result = subprocess.run(
                [
                    "python3", "scripts/nimble_log_watch.py", "--dir", str(log_dir),
                    "--levels", "E", "--include-existing", "--duration-seconds", "1",
                    "--color", "never",
                ],
                cwd=ROOT, capture_output=True, text=True, timeout=10,
            )
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn("lag_bytes=", result.stdout)
        self.assertIn("resumed_at=", result.stdout)
        self.assertIn("recovered record", result.stdout)
        self.assertNotIn("old record", result.stdout)
        self.assertIn("status=parsing", result.stdout)


if __name__ == "__main__":
    unittest.main()
