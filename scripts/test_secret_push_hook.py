"""Offline tests for the local fail-closed pre-push secret scanner.

A fake Gitleaks CLI exercises hook control flow, not real secret detection.
The GitHub Actions Gitleaks step performs the actual detection check.
"""

from __future__ import annotations

import os
import shutil
import subprocess
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
HOOK = ROOT / ".githooks" / "pre-push"
SHELL = next(
    (
        candidate
        for candidate in (
            r"C:\Program Files\Git\usr\bin\sh.exe",
            shutil.which("sh"),
            shutil.which("bash"),
        )
        if candidate and Path(candidate).exists()
    ),
    None,
)

# Single shared fake Gitleaks CLI for every scenario. It logs its arguments and
# validates each --log-opts range with real git (cwd is the repo being pushed),
# so an invalid range such as "<root>^..<root>" for a parentless root commit
# exits 128 exactly like the real CLI. A fake that ignored its arguments could
# not reproduce the root-commit defect offline.
FAKE_GITLEAKS = """#!/bin/sh
printf "%s\\n" "$*" >> "$GITLEAKS_TEST_LOG"
for arg in "$@"; do
  case "$arg" in
    --log-opts=*)
      range="${arg#--log-opts=}"
      git rev-list "$range" >/dev/null 2>&1 || exit 128
      ;;
  esac
done
exit "$GITLEAKS_TEST_EXIT"
"""


class HookTestCase(unittest.TestCase):
    """Shared helpers; each concrete scenario builds its own repos in setUp."""

    def _git(self, *args: str, env: dict | None = None, check: bool = True):
        return subprocess.run(
            ["git", *args],
            cwd=self.repo,
            env=env,
            check=check,
            capture_output=True,
            text=True,
        )

    def _bare_rev(self, ref: str) -> str:
        return subprocess.run(
            ["git", "--git-dir", str(self.bare), "rev-parse", ref],
            check=True,
            capture_output=True,
            text=True,
        ).stdout.strip()

    def _scanned_commits(self) -> list[str]:
        """Ordered tip SHA of every --log-opts range the fake actually scanned."""
        if not self.log_file.exists():
            return []
        scanned: list[str] = []
        for line in self.log_file.read_text(encoding="utf-8").splitlines():
            for token in line.split(" "):
                if token.startswith("--log-opts="):
                    scanned.append(token[len("--log-opts="):].split("..")[-1])
        return scanned

    def _install_hook(self) -> None:
        hooks = self.repo / ".githooks"
        hooks.mkdir()
        shutil.copy2(HOOK, hooks / "pre-push")
        (hooks / "pre-push").chmod(0o755)
        self._git("config", "--local", "core.hooksPath", ".githooks")

    def _install_fake_gitleaks(self, home: Path) -> None:
        self.bin_dir = home / "bin"
        self.bin_dir.mkdir()
        self.log_file = home / "gitleaks.log"
        fake = self.bin_dir / "gitleaks"
        fake.write_text(FAKE_GITLEAKS, encoding="utf-8")
        fake.chmod(0o755)
        self.env = os.environ.copy()
        self.env["PATH"] = str(self.bin_dir) + os.pathsep + self.env.get("PATH", "")
        self.env["GITLEAKS_TEST_LOG"] = str(self.log_file)
        self.env["GITLEAKS_TEST_EXIT"] = "0"


class PrePushHookTests(HookTestCase):
    def setUp(self) -> None:
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        home = Path(self.tmp.name)
        self.repo = home / "working"
        self.bare = home / "remote.git"
        self.repo.mkdir()
        self._git("init", "-b", "main")
        self._git("config", "user.name", "Hook Test")
        self._git("config", "user.email", "hook@example.invalid")
        (self.repo / "sample.txt").write_text("initial\n", encoding="utf-8")
        self._git("add", "sample.txt")
        self._git("commit", "-m", "initial")
        subprocess.run(
            ["git", "init", "--bare", str(self.bare)],
            check=True,
            capture_output=True,
            text=True,
        )
        self._git("remote", "add", "origin", str(self.bare))
        self._git("push", "--no-verify", "-u", "origin", "main")
        self.old_remote = self._bare_rev("refs/heads/main")

        self._install_hook()
        self._install_fake_gitleaks(home)

        (self.repo / "sample.txt").write_text("new change\n", encoding="utf-8")
        self._git("add", "sample.txt")
        self._git("commit", "-m", "new commit")

    def test_clean_push_scans_new_commits(self) -> None:
        result = self._git("push", "origin", "main", env=self.env)
        self.assertIn("Scanning commit", result.stderr)
        self.assertIn("--log-opts=", self.log_file.read_text(encoding="utf-8"))
        self.assertEqual(
            self._bare_rev("refs/heads/main"),
            self._git("rev-parse", "HEAD").stdout.strip(),
        )

    def test_gitleaks_finding_blocks_remote_push(self) -> None:
        self.env["GITLEAKS_TEST_EXIT"] = "1"
        result = self._git("push", "origin", "main", env=self.env, check=False)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("Secret scan BLOCKED", result.stderr)
        self.assertEqual(self._bare_rev("refs/heads/main"), self.old_remote)

    def test_new_branch_scans_only_unpushed_commits(self) -> None:
        self._git("checkout", "-b", "feature")
        self._git("push", "origin", "feature", env=self.env)
        new_commit = self._git("rev-parse", "HEAD").stdout.strip()
        scanned = self._scanned_commits()
        self.assertEqual(scanned, [new_commit])
        self.assertNotIn(self.old_remote, scanned)
        self.assertEqual(self._bare_rev("refs/heads/feature"), new_commit)

    def test_missing_gitleaks_blocks_push(self) -> None:
        env = self.env.copy()
        env["PATH"] = os.pathsep.join(
            path
            for path in self.env["PATH"].split(os.pathsep)
            if path
            and not (Path(path) / "gitleaks").exists()
            and not (Path(path) / "gitleaks.exe").exists()
        )
        result = self._git("push", "origin", "main", env=env, check=False)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("install Gitleaks", result.stderr)

    def test_unavailable_remote_base_blocks_push(self) -> None:
        result = subprocess.run(
            [SHELL, str(HOOK), "origin", str(self.bare)],
            check=False,
            cwd=self.repo,
            env=self.env,
            text=True,
            capture_output=True,
            input="refs/heads/main "
            + self._git("rev-parse", "HEAD").stdout.strip()
            + " refs/heads/main "
            + ("1" * 40)
            + "\n",
        )
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("cannot inspect remote base", result.stderr)
        self.assertFalse(self.log_file.exists())

    def test_multiple_pushed_refs_are_scanned_independently(self) -> None:
        self._git("checkout", "-b", "second")
        result = self._git("push", "origin", "main", "second", env=self.env)
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(len(self.log_file.read_text(encoding="utf-8").splitlines()), 2)

    def test_commit_introduced_then_deleted_is_still_scanned(self) -> None:
        new_commit = self._git("rev-parse", "HEAD").stdout.strip()
        (self.repo / "temporary.txt").write_text("synthetic-secret\n", encoding="utf-8")
        self._git("add", "temporary.txt")
        self._git("commit", "-m", "add synthetic secret")
        secret_add = self._git("rev-parse", "HEAD").stdout.strip()
        self._git("rm", "temporary.txt")
        self._git("commit", "-m", "remove synthetic secret")
        secret_remove = self._git("rev-parse", "HEAD").stdout.strip()
        # Clean fake exit so every new commit is scanned, proving the
        # introduced-then-deleted secret commit is reached rather than skipped.
        self.env["GITLEAKS_TEST_EXIT"] = "0"
        result = self._git("push", "origin", "main", env=self.env)
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(
            self._scanned_commits(), [new_commit, secret_add, secret_remove]
        )
        self.assertEqual(self._bare_rev("refs/heads/main"), secret_remove)

    def test_ref_deletion_does_not_invoke_gitleaks(self) -> None:
        result = subprocess.run(
            [SHELL, str(HOOK), "origin", str(self.bare)],
            check=False,
            cwd=self.repo,
            env=self.env,
            text=True,
            capture_output=True,
            input="refs/heads/deleted "
            + ("0" * 40)
            + " refs/heads/deleted "
            + self.old_remote
            + "\n",
        )
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertFalse(self.log_file.exists())


class RootCommitPushTests(HookTestCase):
    """F2 regression: first push of a repo whose only commit is a parentless root."""

    def setUp(self) -> None:
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        home = Path(self.tmp.name)
        self.repo = home / "working"
        self.bare = home / "remote.git"
        self.repo.mkdir()
        self._git("init", "-b", "main")
        self._git("config", "user.name", "Hook Test")
        self._git("config", "user.email", "hook@example.invalid")
        (self.repo / "sample.txt").write_text("synthetic-secret\n", encoding="utf-8")
        self._git("add", "sample.txt")
        self._git("commit", "-m", "root commit")
        self.root_commit = self._git("rev-parse", "HEAD").stdout.strip()
        subprocess.run(
            ["git", "init", "--bare", str(self.bare)],
            check=True,
            capture_output=True,
            text=True,
        )
        self._git("remote", "add", "origin", str(self.bare))
        # Nothing is pre-pushed: pushing main is a new-ref push of a root commit.
        self._install_hook()
        self._install_fake_gitleaks(home)

    def test_root_commit_clean_push_succeeds(self) -> None:
        result = self._git("push", "origin", "main", env=self.env, check=False)
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(self._bare_rev("refs/heads/main"), self.root_commit)
        self.assertEqual(self._scanned_commits(), [self.root_commit])

    def test_root_commit_with_secret_blocks(self) -> None:
        self.env["GITLEAKS_TEST_EXIT"] = "1"
        result = self._git("push", "origin", "main", env=self.env, check=False)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("Secret scan BLOCKED", result.stderr)
        bare_ref = subprocess.run(
            ["git", "--git-dir", str(self.bare), "for-each-ref", "refs/heads/main"],
            check=True,
            capture_output=True,
            text=True,
        ).stdout.strip()
        self.assertEqual(bare_ref, "")


if __name__ == "__main__":
    unittest.main()
