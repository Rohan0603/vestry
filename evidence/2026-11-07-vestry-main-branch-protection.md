# Vestry `main` branch protection — application and enforcement demonstration

- **Date:** 7 November 2026 (commands executed 07:35–07:43 UTC)
- **Decision key:** `vestry-ves4-main-protection-20261107`
- **Issue:** VES-4 · **Baseline authority:** §21 (D35), D32 (`target` vs `verified-enforced`)
- **Authorization:** Owner, 7 November 2026 — "Authorize me to apply" (branch-protection decision),
  followed by the Owner making the repository public ("done made it public"), which removed the
  GitHub free-plan private-repository restriction that had blocked the API with HTTP 403.

## Environment

- Repository: `Rohan0603/vestry`, visibility **public** (confirmed via
  `gh api repos/Rohan0603/vestry` → `{"default_branch":"main","private":false,"visibility":"public"}`).
- Actor: `Rohan0603` — repository **admin** and sole collaborator. This matters: every rejection
  below was issued to the admin account, so admin enforcement is part of what was demonstrated.
- Tooling: `gh` CLI (GitHub REST API v3), Git for Windows. Local clone `C:/Users/ponna/Project/vestry`.
- `main` before and after all tests: `f4c83ee79c818e4ef103a3f22a68bf277620dd0a` (unchanged; verified by
  `git ls-remote origin refs/heads/main` after the last test).

## What was applied

`PUT repos/Rohan0603/vestry/branches/main/protection` (payload: `required_status_checks {strict: true,
checks: []}`, `required_pull_request_reviews {dismiss_stale_reviews: true,
required_approving_review_count: 1, require_code_owner_reviews: false}`, `enforce_admins: true`,
`restrictions: null`, `allow_force_pushes: false`, `allow_deletions: false`, all other flags false).
The PUT returned 200 with the settings echoed.

**Independent read-back** (separate `GET repos/Rohan0603/vestry/branches/main/protection`, after the
PUT) confirmed, verbatim from the response:

```json
"required_status_checks": {"strict": true, "contexts": [], "checks": []}
"required_pull_request_reviews": {"dismiss_stale_reviews": true, "require_code_owner_reviews": false,
  "require_last_push_approval": false, "required_approving_review_count": 1}
"enforce_admins": {"enabled": true}
"allow_force_pushes": {"enabled": false}
"allow_deletions": {"enabled": false}
"required_signatures": {"enabled": false}
"required_linear_history": {"enabled": false}
"block_creations": false, "required_conversation_resolution": false,
"lock_branch": false, "allow_fork_syncing": false
```

## Demonstrations actually executed

### 1. Direct push to `main` — REJECTED (PASS)

A throwaway commit object was created without touching the working tree:
`git commit-tree -p f4c83ee… -m "VES-4 protection demonstration: direct push attempt (expected rejection)" <tree of origin/main>`
→ `6ea0da24d95d665125aab89a0a1289260a3aed71` (a fast-forward child of `main`, identical tree).

```
git push origin 6ea0da24d95d665125aab89a0a1289260a3aed71:refs/heads/main
remote: error: GH006: Protected branch update failed for refs/heads/main.
remote: - Changes must be made through a pull request.
 ! [remote rejected] 6ea0da2… -> main (protected branch hook declined)
EXIT=1
```

A throwaway local branch created for the test was deleted afterwards; the dangling commit object was
never pushed and is unreachable. `main` verified unchanged.

### 2. Force push to `main` — REJECTED (PASS)

Non-fast-forward source (`main~1` = `916adfdcf94f1f211344e8c68f16235f0e18fe60`), so a same-SHA
"Everything up-to-date" no-op could not masquerade as a test:

```
git push --force origin 916adfdcf94f1f211344e8c68f16235f0e18fe60:refs/heads/main
remote: error: GH006: Protected branch update failed for refs/heads/main.
remote: - Changes must be made through a pull request.
remote: - Cannot force-push to this branch
 ! [remote rejected] 916adfd… -> main (protected branch hook declined)
EXIT=1
```

### 3. Unreviewed PR cannot merge — BLOCKED state observed (PASS, state-level)

> **Supplement (7 November 2026, later commit):** the merge attempt this section deferred was
> subsequently executed and rejected by the base-branch policy — see
> [`2026-11-07-vestry-pr1-unreviewed-merge-attempt.md`](2026-11-07-vestry-pr1-unreviewed-merge-attempt.md).
> The original text below is unchanged and was true when written.

`gh pr view 1 --json mergeable,mergeStateStatus,state,reviews` →
PR #1 (base `main`, head `6ce03c145462bcdb5d35a56075bae3235e834a95`):
`state: OPEN`, `mergeable: MERGEABLE`, `mergeStateStatus: BLOCKED`, **zero reviews**.
GitHub computes the merge as blocked solely for lack of the required approving review.
**No merge was attempted** — the blocked state is the evidence; attempting a merge that might
succeed was outside authorization.

### 4. Admin enforcement — demonstrated implicitly (PASS)

Both push rejections (tests 1 and 2) were issued to `Rohan0603`, the repository admin. With
`enforce_admins: true` read back, the admin account is demonstrably subject to the same rules.

## Not run — recorded gaps (D29: NOT RUN, with reasons)

| Check | Configured state | Why NOT RUN |
|---|---|---|
| Stale-approval dismissal on new commits | `dismiss_stale_reviews: true` | No approving review exists to dismiss. The sole collaborator is the author of every PR commit, and GitHub forbids self-approval — a second reviewer is a structural prerequisite. |
| Strict up-to-date requirement (`strict: true`) | configured | `main` has not advanced beyond the open PRs' merge bases; demonstrating the block requires an authorized merge to advance `main` first. |
| Required status checks | `checks: []` | No CI exists yet; nothing to require. Remains a `target` until a workflow is added and its check is bound. |
| Branch-deletion block | `allow_deletions: false` | No deletion of `main` was attempted: the blast radius if the setting were wrong is the loss of the base branch. Read-back only. |
| Merge-time review enforcement | 1 approving review required | Evidence is the `BLOCKED` merge state (test 3); no actual merge call was made. *[Erratum: a merge call was later made and rejected — see the supplement linked at test 3; this row is superseded.]* |
| Signed-commits requirement | `required_signatures: false` | Not enabled; §21 says "signed where the toolchain allows" — deferred to release tooling work. |

## D29 summary

- **PASS (high confidence):** protection applied and read back; direct push rejected; force push
  rejected; unreviewed PR merge blocked at state level; admin enforcement observed. All four are
  server-side rejections/observations on the live repository, not local inference.
- **NOT RUN:** the six checks tabulated above, each with its reason. Their configured values are
  read-back-verified; their *enforcement* is not demonstrated and they remain `target` per D32.
- **Confidence qualifier:** every PASS was measured against GitHub's hosted enforcement for a public
  repository on the free plan. No claim is made about behaviour under paid-plan features, rulesets,
  or a future migration to organization ownership.

## Structural blocker surfaced (not resolved by this activity)

With one collaborator who is also the commit author of all open PRs, GitHub cannot record the
independent non-author approving review §21 requires. PRs #1/#2/#3 are therefore unmergeable until
the Owner adds a reviewer or records a superseding decision. This is an Owner decision, tracked in
VES-4's Linear comments; nothing in this evidence weakens the review requirement to route around it.

## Redaction and traceability

No credential, token, or sensitive payload appears in this record; commands and outputs contain only
public SHAs, branch names, and API paths. Redaction check: PASS (manual scan, 7 November 2026).
Traceability: `Refs: VES-4` commit footer → PR → this file; Linear comment on VES-4 carries the same
decision key. Supersedes the blocker recorded in Linear comment `693c19ce-f4b2-426b-8e3d-3b309872ab23`
(HTTP 403 on private free plan), which remains historically true.