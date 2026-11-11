# Secret scanning and push protection

**Policy:** Never commit API keys, access tokens, credentials, private keys, passwords or real `.env` files. Use `.env.example` containing placeholders and store actual secrets in approved local/CI secret stores.

## Developer workstation (Git Bash, Linux, macOS, Windows)

1. Install the maintained [Gitleaks CLI](https://github.com/gitleaks/gitleaks) and ensure `gitleaks` is in PATH.
2. In each clone, run `git config --local core.hooksPath .githooks` (works in Git Bash on Windows and in Linux/macOS shells). `gitleaks` must be in PATH; the hook fails closed if it is missing.
3. The repository-owned `.githooks/pre-push` scans all *new commits* in every pushed ref against the destination remote. It **blocks the push** if Gitleaks is missing, cannot determine the remote range, or detects a secret. It does not modify commits or upload their contents.
4. A hook is **not installed merely by cloning a repository**, and local hooks are bypassable (`--no-verify`). GitHub push protection and a required CI check provide additional safeguards.
5. If any secret has already been committed, removing it from the working tree or a later commit does not remove it from history. Revoke/rotate exposed credentials immediately; plan any history rewrite separately.

## GitHub server-side checks

- `.github/workflows/secret-scan.yml` downloads the pinned Gitleaks `8.30.1` Linux CLI, verifies the published SHA-256 checksum before execution, runs the real synthetic-secret regression, and scans pull-request, `main`/`master` push, and manual-dispatch history. It does not require `GITLEAKS_LICENSE` and does not start any AI agent.
- On supported branches, set **Gitleaks secret scan** as a required status check **after the first successful workflow run**, using the exact name GitHub reports. Do not invent or preconfigure a missing check.
- Enable GitHub **Secret scanning / Push protection** under **Settings → Security and quality → Advanced Security**, if supported. GitHub's user-level push protection also covers supported secrets pushed to public repositories. Protection availability differs for private repositories and may depend on plan/entitlements.
- Neither the hook nor CI is proof that all secret types were recognized. Review unusual custom tokens and credentials manually; restrict GitHub Actions permissions.
- `.gitignore` only stops untracked files from being added; it does not untrack a previously committed `.env`, and it does not remove historical blobs.

## Repository adoption

Copy this hook and the workflow to each applicable repository through a reviewed PR. On Windows, activate the local hook **in every clone** after pulling the policy by running `git config --local core.hooksPath .githooks`. For protected branches, run CI, perform the required review, and merge under normal branch rules. For private repos without native GitHub push protection, local Gitleaks plus CI is a useful compensating control, not equivalent to native server-side rejection.

For existing exposed secrets, resolve provider revocation and safe history remediation before claiming the incident closed.
