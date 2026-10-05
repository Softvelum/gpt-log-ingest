"""Portable repository checks; these never launch Nimble or an AI client."""

from pathlib import Path
import json
import re
import subprocess
import unittest
from urllib.parse import unquote, urlsplit


ROOT = Path(__file__).resolve().parents[1]
SKILLS = [
    ROOT / ".agents/skills/nimble-logs/SKILL.md",
    ROOT / ".claude/skills/nimble-logs/SKILL.md",
]


def local_links(path):
    """Resolve Markdown file links relative to the document containing them."""
    for target in re.findall(r"\[[^\]]*\]\(([^)]+)\)", path.read_text()):
        parsed = urlsplit(target)
        if parsed.scheme or parsed.netloc or not parsed.path:
            continue
        yield (path.parent / unquote(parsed.path)).resolve()


class RepositoryTests(unittest.TestCase):
    def test_skills_have_standard_discovery_metadata(self):
        for path in SKILLS:
            with self.subTest(skill=path.relative_to(ROOT)):
                match = re.match(r"\A---\n(.*?)\n---\n", path.read_text(), re.S)
                self.assertIsNotNone(match, "Missing skill frontmatter")
                fields = dict(line.split(":", 1) for line in match[1].splitlines())
                self.assertEqual(fields["name"].strip(), path.parent.name)
                self.assertRegex(fields["name"].strip(), r"^[a-z0-9-]+$")
                description = fields["description"].strip()
                self.assertTrue(description)
                self.assertLessEqual(len(description), 1024)
                self.assertNotRegex(description, r"[<>]")

    def test_claude_skill_directory_links_to_canonical_agents_skill(self):
        claude_skill_dir = SKILLS[1].parent
        self.assertTrue(claude_skill_dir.is_symlink(), "Claude skill must link to the canonical skill directory")
        self.assertEqual(claude_skill_dir.resolve(), SKILLS[0].parent.resolve())
        self.assertTrue(SKILLS[1].is_file())
        self.assertEqual(SKILLS[0].read_bytes(), SKILLS[1].read_bytes())

    def test_both_skills_route_to_shared_analysis_and_tests(self):
        expected = {
            ROOT / "workflows/analyze-logs.md",
            ROOT / "workflows/test-workspace.md",
        }
        for path in SKILLS:
            with self.subTest(skill=path.relative_to(ROOT)):
                targets = set(local_links(path))
                self.assertTrue(expected <= targets, "Missing shared workflow route")
                for target in expected:
                    self.assertTrue(target.is_file(), f"Missing workflow: {target}")

    def test_watch_workflow_requires_live_polling_and_chat_relay(self):
        workflow = (ROOT / "workflows/analyze-logs.md").read_text().lower()
        for phrase in (
            "poll the running watcher tool",
            "keep the agent turn active",
            "relay each heartbeat and batch",
            "no push channel",
            "session ends",
        ):
            with self.subTest(phrase=phrase):
                self.assertIn(phrase, workflow)
        skill = SKILLS[0].read_text().lower()
        self.assertIn("poll the running watcher tool session", skill)
        self.assertIn("no push channel", skill)
        readme = re.sub(r"\s+", " ", (ROOT / "README.md").read_text().lower())
        self.assertIn("poll that session", readme)
        self.assertIn("relay status and findings into the conversation", readme)

    def test_public_document_links_resolve(self):
        files = [ROOT / "README.md", ROOT / "AGENTS.md", *SKILLS]
        for directory in ["workflows", "templates", "docs/superpowers", "tests"]:
            files.extend((ROOT / directory).rglob("*.md"))
        for path in files:
            for target in local_links(path):
                with self.subTest(document=path.relative_to(ROOT), target=str(target)):
                    self.assertTrue(target.exists(), "Broken document reference")

    def test_claude_imports_shared_repository_instructions(self):
        self.assertEqual((ROOT / "CLAUDE.md").read_text().strip(), "@AGENTS.md")

    def test_catalog_index_covers_each_message_once(self):
        index = list(local_links(ROOT / "logger/README.md"))
        entries = [p for p in index if p.name.startswith("log_")]
        catalog = set((ROOT / "logger").glob("log_*.md"))
        self.assertEqual(len(entries), len(set(entries)), "Duplicate index entries")
        self.assertEqual(set(entries), catalog, "Missing or obsolete index entries")
        for path in entries:
            self.assertTrue(path.is_file(), f"Missing catalog entry: {path.name}")

    def test_generated_logger_search_index_is_current(self):
        result = subprocess.run(
            ["python3", "scripts/build_logger_map.py", "--check"],
            cwd=ROOT, capture_output=True, text=True, check=False,
        )
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)

    def test_local_settings_launch_parameters_and_reports_stay_private(self):
        paths = [
            ".local/settings.md", ".local/user-instructions.md",
            ".local/nimble-test.json", ".local/state.md", ".local/reports/test.md",
        ]
        result = subprocess.run(
            ["git", "check-ignore", "--no-index", *paths], cwd=ROOT,
            capture_output=True, text=True, check=False,
        )
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(set(result.stdout.splitlines()), set(paths))
        tracked = subprocess.run(
            ["git", "ls-files", "--", ".local"], cwd=ROOT,
            capture_output=True, text=True, check=True,
        )
        self.assertEqual(tracked.stdout.strip(), "", "User-local files are tracked")

    def test_commit_security_review_follows_all_repository_checks(self):
        guidance = " ".join((ROOT / "AGENTS.md").read_text().split())
        test_step = guidance.index("5. Run `python3 -m unittest discover")
        map_step = guidance.index("6. Run `python3 scripts/build_logger_map.py")
        security_step = guidance.index(
            "7. After all repository, behavior, map, and Rust tests"
        )
        self.assertLess(test_step, security_step)
        self.assertLess(map_step, security_step)
        for phrase in (
            "full working-tree security review",
            ".pem",
            "passwords",
            "personal notes",
            "actual Nimble logs",
            "without printing secret values",
        ):
            with self.subTest(phrase=phrase):
                self.assertIn(phrase, guidance)

        workflow = (ROOT / "workflows/test-workspace.md").read_text()
        report_step = workflow.index("## 5. Report the test result")
        workflow_security_step = workflow.index(
            "## 6. Review repository contents for secrets before commit"
        )
        self.assertLess(report_step, workflow_security_step)
        self.assertIn("hidden or ignored", workflow[workflow_security_step:].lower())

    def test_launch_template_has_separate_executable_cwd_and_arguments(self):
        path = ROOT / "templates/nimble-test.json"
        self.assertTrue(path.is_file(), "Missing local test-launch template")
        profile = json.loads(path.read_text())
        self.assertEqual(profile["mode"], "launch")
        self.assertIsInstance(profile["executable"], str)
        self.assertIsInstance(profile["working_directory"], str)
        self.assertIsInstance(profile["arguments"], list)
        self.assertTrue(all(isinstance(arg, str) for arg in profile["arguments"]))
        self.assertGreater(profile["duration_seconds"], 0)
        self.assertTrue(profile["stop_after_test"])


if __name__ == "__main__":
    unittest.main()
