"""whep-check feature checks; these never contact a WHEP endpoint."""

from pathlib import Path
import unittest

from test_repository import local_links


ROOT = Path(__file__).resolve().parents[1]
SKILL = ROOT / ".agents/skills/whep-check/SKILL.md"
WORKFLOW = ROOT / "workflows/check-whep.md"
TOOL = ROOT / "tools/whep-check"


class WhepCheckFeatureTests(unittest.TestCase):
    def test_skill_routes_to_its_workflow_and_client(self):
        self.assertIn(WORKFLOW, set(local_links(SKILL)))
        for name in ("go.mod", "go.sum", "main.go", "main_test.go", "stats.go", "output.go", "README.md"):
            with self.subTest(file=name):
                self.assertTrue((TOOL / name).is_file())

    def test_workflow_keeps_its_boundaries(self):
        text = " ".join(WORKFLOW.read_text().lower().split())
        for phrase in ("explicit request", "must not run it", "delete", "redact"):
            with self.subTest(phrase=phrase):
                self.assertIn(phrase, text)


if __name__ == "__main__":
    unittest.main()
