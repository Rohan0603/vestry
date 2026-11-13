# Vestry `main` branch protection — current required-check and strict up-to-date enforcement

- **Date:** 14 November 2026 (commands executed 13 November 2026, 22:23Z–22:43Z UTC, and re-verified 14 November 2026)
- **Card:** [Vestry #8 / VES-4](https://trello.com/c/eePm8vuW) · authorizing card [GgTpbAhr](https://trello.com/c/GgTpbAhr)
- **Decision context:** D29/D31 (PASS/FAIL/NOT RUN, never weaken a criterion), D32 (a control is `target` until demonstrated), D39/D43 (independent exact-SHA review; Owner standing merge authorization), D40(c) (required checks), D49(b) (authorized demonstration window)
- **Repository:** `mission-control-panel/vestry`
- **Authorisation:** Owner, 2026-11-14, "yes for all" (recorded on `GgTpbAhr`); PM delegation T32b.

This file is self-contained. It replaces the untracked working records `migration-work/T30-VERIFICATION-WINDOW.md` and `migration-work/T32-R3-ATTEMPT.md` as the durable record; nothing in this evidence depends on those files.

## 1. What this record establishes

Three branch-protection controls on `main` were demonstrated by live, server-side rejection, each isolated from the other merge gates:

| Control | Demonstration | Result |
|---|---|---|
| Direct push to `main` is rejected | Fast-forward child commit pushed to `refs/heads/main` (R1) | PASS — GH006, exit 1, `main` unchanged |
| The current required-check pair blocks a merge | PR #55 with a deliberate defect: `Policy validator tests` fail, `Gitleaks secret scan` pass (R4) | PASS — blocked, exit 1, isolated |
| A branch that is not up to date cannot merge | PR #56 with `mergeStateStatus: BEHIND`, both required checks green (R3) | PASS — blocked with the gate named, exit 1, isolated |

`CONTRIBUTING.md` rows 116 (required-check pair) and 117 (strict up-to-date) move from `target` to `verified-enforced` in the same change that adds this file, as that table requires. No other status changes: the dismiss-stale row stays `target` (moot under D39; its acceptance box is recorded as superseded, never PASS, D50), and the deletion, signature and update-branch behaviours were **not** measured (§6).

## 2. Environment and configuration at evidence time

- Windows 10.0.26200, Git Bash; `gh` CLI authenticated as `Rohan0603`, the repository **admin** and sole collaborator — every rejection below was issued to the admin account, so admin enforcement is part of what was shown.
- Node v24.22.1 for the governance checker; Python offline tests for the validator.
- Branch work for R1/R4/R3 was done in throwaway branches/worktrees; the shared working tree was not switched for the demos.
- Protection read back on 14 November 2026 (`GET repos/mission-control-panel/vestry/branches/main/protection`), unchanged throughout the demos:

```json
{"contexts":["Policy validator tests","Gitleaks secret scan"],"conversation":true,
 "deletions":false,"dismiss_stale_reviews":true,"enforce_admins":true,"force_push":false,
 "required_approving_review_count":0,"signatures":false,"strict":true}
```

- `main` sequence: `2c0abd19fbddb36f41ad209e6729650ec74b8fd4` for R1 and R4; `08b979c72ec99c46d24e2affaffbad3750d78403` for R3 and its post-merge CI; `72f5099215ca0dbef4df0cd8f982b5bd093ed8fb` (the PR #58 merge) when this record was written.

## 3. R1 — direct push to `main` rejected

No working-tree change: a fast-forward child of `main` with an identical tree was created as a bare commit object.

```
PRE_SHA=2c0abd19fbddb36f41ad209e6729650ec74b8fd4
CANDIDATE=0f9f55733656fbbd27378ba0c3da053b667c9756
git push origin 0f9f55733656fbbd27378ba0c3da053b667c9756:refs/heads/main
```

Verbatim output:

```
remote: error: GH006: Protected branch update failed for refs/heads/main.
remote:
remote: - Changes must be made through a pull request.
remote:
remote: - 2 of 2 required status checks are expected.
To https://github.com/mission-control-panel/vestry.git
 ! [remote rejected] 0f9f55733656fbbd27378ba0c3da053b667c9756 -> main (protected branch hook declined)
error: failed to push some refs to 'https://github.com/mission-control-panel/vestry.git'
EXIT=1
```

Post-check `git ls-remote origin refs/heads/main` returned `2c0abd19fbddb36f41ad209e6729650ec74b8fd4` — unchanged.

**Verdict: PASS.** The rejection is server-side, issued to the admin account. The wording now additionally cites the required-check gate ("2 of 2 required status checks are expected"); it is therefore not byte-identical to the 7 November record (`evidence/2026-11-07-vestry-main-branch-protection.md` §1), but the outcome is the same and stronger. The R1 timestamp was not captured in the source window record; it executed before the R4 PR was opened at 22:23Z.

## 4. R4 — the current required-check pair blocks a merge (isolated)

Throwaway branch `ves4-demo-checkfail` carried one new file `demo-check-violation.md` with a deliberate trailing-whitespace line (line 8). The same checker run locally reproduces the failure:

```
governance: FAIL (1 violation(s))
  - demo-check-violation.md:8: trailing whitespace
EXIT=1
```

PR [#55](https://github.com/mission-control-panel/vestry/pull/55) — title `DO NOT MERGE — VES-4 required-check enforcement demo (expected FAIL)` — head `77d400d862317c889e7f6629c00f1c222646f0d0`, base `2c0abd19fbddb36f41ad209e6729650ec74b8fd4`.

| Check | Result | Run / job |
|---|---|---|
| `Policy validator tests` | **fail** (21 s) | run 43518904528 / job 132207866157 |
| `Gitleaks secret scan` | pass (5 s) | run 43518904582 / job 132207865792 |
| `governance` (aggregate) | fail (5 s) | run 43518904528 / job 132208059815 |

Failing-step log:

```
Policy validator tests  Validate documents and preserve prior decision rows  governance: FAIL (1 violation(s))
Policy validator tests  Validate documents and preserve prior decision rows    - demo-check-violation.md:8: trailing whitespace
Policy validator tests  Validate documents and preserve prior decision rows  ##[error]Process completed with exit code 1.
governance  Require offline checks  ##[error]Process completed with exit code 1.
```

`gh pr view 55 --json mergeStateStatus` returned `BLOCKED`. Merge attempt (no `--admin`, no `--auto`):

```
gh pr merge 55 --squash
X Pull request mission-control-panel/vestry#55 is not mergeable: the base branch policy prohibits the merge.
To have the pull request merged after all the requirements have been met, add the `--auto` flag.
To use administrator privileges to immediately merge the pull request, add the `--admin` flag.
MERGE_EXIT=1
```

Post-attempt: `state: OPEN`, `mergeStateStatus: BLOCKED`, `mergedAt: null`; `main` still `2c0abd1`. Cleanup: PR #55 closed unmerged, branch `ves4-demo-checkfail` deleted and verified absent; current state re-checked on 14 November: `CLOSED`, `mergedAt: null`.

**Verdict: PASS, isolated.** `required_approving_review_count` is 0, no conversation was unresolved, and `Gitleaks secret scan` passed; the failing `Policy validator tests` check (and its aggregate) was the only unsatisfied gate. This is the isolation the historical PR #6 lacked. It demonstrates the *current* required-check pair, not the former single `governance` context.

## 5. R3 — strict up-to-date blocks a merge (isolated)

Preconditions read immediately before the attempt: `origin/main` `08b979c…`; PR [#56](https://github.com/mission-control-panel/vestry/pull/56) `OPEN`, `mergeable: MERGEABLE`, head `ff258cb196a31b1f0c341a4abca3731d965d1e51`, base ref `2c0abd19fbddb36f41ad209e6729650ec74b8fd4`, **`mergeStateStatus: BEHIND`** (the required precondition; the attempt would have aborted on `CLEAN`/`UNKNOWN`).

Required checks re-read on the demo head:

```
governance             success  run 43519133047 / job 132208737123
Policy validator tests success  run 43519133047 / job 132208644370
Gitleaks secret scan   success  run 43519133103 / job 132208643223
```

Attempt (verbatim):

```
START=2026-11-13T22:42:58Z
gh pr merge 56 --repo mission-control-panel/vestry --squash
--- verbatim stdout/stderr ---
X Pull request mission-control-panel/vestry#56 is not mergeable: the head branch is not up to date with the base branch.
To have the pull request merged after all the requirements have been met, add the `--auto` flag.
To use administrator privileges to immediately merge the pull request, add the `--admin` flag.
--- exit ---
EXIT=1
END=2026-11-13T22:42:59Z
```

`--auto` and `--admin` were not passed; the two lines above are GitHub's hint text, not commands run.

Post-attempt state:

```
git ls-remote origin refs/heads/main
08b979c72ec99c46d24e2affaffbad3750d78403	refs/heads/main

gh pr view 56 --json state,mergeStateStatus,mergedAt
{"mergeStateStatus":"BEHIND","mergedAt":null,"state":"OPEN"}
```

Cleanup: PR #56 closed (`closedAt 2026-11-13T22:43:13Z`), remote branch `ves4-demo-strict2` deleted and verified absent; current state re-checked on 14 November: `CLOSED`, `mergedAt: null`.

**Verdict: PASS, isolated.** The rejection names the gate explicitly ("the head branch is not up to date with the base branch"). Because approvals are 0, no conversation was unresolved and both required checks were green on the head, strict up-to-date was the only unsatisfied merge gate. This differs from the historical PR #48 merge, where a branch that was ancestry-behind but content-current merged `CLEAN`.

## 6. Post-merge CI on `08b979c`

Both required workflows re-ran on the current `main` after the rev-1.9 merge:

```
43520376229  Agent Governance CI  completed  success  head_sha 08b979c…  push  22:34:26Z
43520376400  Secret Scan          completed  success  head_sha 08b979c…  push  22:34:26Z
```

## 7. Rev-1.9 merge-sequence deviation (process, not content)

Two merges of the revision-1.9 work occurred before any recorded D39 verdict existed. The API records only the account, and cannot distinguish a human from an agent using it.

**PR #57 (revision 1.9, D47–D50).** Merged `2026-11-13T22:34:24Z` by account `Rohan0603`, merge commit `08b979c72ec99c46d24e2affaffbad3750d78403`, base `2c0abd1`, reviewed head `1fcddbeee7e3f1e973310e8324e2d2ff082fec7e`. The D39 review was delegated at `22:34:37Z` — after the merge — and the PR had zero prior review comments. No team member held a merge assignment; the T29/T30/T31 final reports each state no member merged it. This is consistent with an Owner direct merge, the same pattern recorded for PR #53 (D47). The later verdict ([comment 6866692639](https://github.com/mission-control-panel/vestry/pull/57#issuecomment-6866692639)) is an APPROVE of the exact reviewed head and records that the merged tree `ea3b34d32b0536dbbd45cd36a8cfa57d40237b2b` equals that head's tree, so it binds the merged bytes; the verdict-before-merge sequence itself was not observed.

**PR #58 (revision 1.9 errata and pointer sync).** Merged `2026-11-13T22:51:06Z` (committer `web-flow`, i.e. a GitHub merge), merge commit `72f5099215ca0dbef4df0cd8f982b5bd093ed8fb`, head `b691fa699a7a9fef07acd8a2a0f5dda8ce6cf04b`. The PR has **zero comments and zero submitted reviews**; a D39 review of this PR had been assigned to the Code Reviewer at `22:49:23Z` (recorded on `GgTpbAhr`), 1 minute 43 seconds before the merge, so no verdict could have existed at merge time. No D39 verdict for #58 was recorded as of this writing.

The deviation was recorded by the PM on `GgTpbAhr` in comment `6af79248a1a9e938a9c0308b` (`2026-11-13T22:42:16Z`), which states that the merge actor is consistent with the Owner, that the GitHub API cannot distinguish human from agent use of the account, and that Owner confirmation is pending. That confirmation is still pending for both merges. The records of #57 and #58 remain as merged; no content rework is implied for #57, and a covering review of #58 is outstanding.

This section is a provenance record. It does not affect the R1/R4/R3 enforcement results, which concern GitHub server-side gates rather than the review sequence, and it does not convert the D39 review control (`CONTRIBUTING.md:112`) from `process-bound`.

## 8. Exact commands

```text
git push origin 0f9f55733656fbbd27378ba0c3da053b667c9756:refs/heads/main
git ls-remote origin refs/heads/main
node scripts/ci/governance-checks.mjs                       # local reproduction on the R4 demo branch
gh pr view 55 --json state,mergeStateStatus,mergedAt
gh pr merge 55 --squash
gh pr view 56 --json state,mergeStateStatus,mergedAt
gh pr merge 56 --repo mission-control-panel/vestry --squash
gh pr close 55 --repo mission-control-panel/vestry
gh pr close 56 --repo mission-control-panel/vestry
git push origin --delete ves4-demo-checkfail
git push origin --delete ves4-demo-strict2
gh run view 43518904528 --json conclusion,headSha,databaseId
gh run view 43518904582 --json conclusion,headSha,databaseId
gh run view 43519133047 --json conclusion,headSha,databaseId
gh run view 43519133103 --json conclusion,headSha,databaseId
gh run view 43520376229 --json conclusion,headSha,databaseId
gh run view 43520376400 --json conclusion,headSha,databaseId
gh api repos/mission-control-panel/vestry/branches/main/protection
```

## 9. What was not measured

- **Merge behaviour after "Update branch".** The demo branches were closed rather than updated.
- **Content-current-but-ancestry-behind branches.** PR #48 remains the observation that such a branch merged `CLEAN`; R3 used a branch whose decision rows/contents were behind, so it is a strict-block of the content-behind case, not of the content-current case.
- **Deletion of `main`.** Configured (`allow_deletions: false`) but not attempted — blast radius.
- **Commit signatures.** `required_signatures: false`; not tested.
- **UI-level wording and behaviour.** Only CLI/API outcomes were recorded.
- **Any other protection setting, repository or organisation ruleset.** None was changed or tested.
- **Human-versus-agent attribution** of either rev-1.9 merge. Only the account is observable.
- **Trello write-probe behaviours** (attachments, 429 handling, forged markers, second-actor discrimination) — outside this record's scope.

## 10. Confidence

- R1, R4, R3: **high** — live server-side rejections, verbatim outputs, exit codes, run ids and unchanged `main` read back; R4 and R3 are additionally isolated from the review and other check gates by approvals 0, no unresolved conversation and the complementary check passing.
- Post-merge CI: **high** — run ids, head SHA and conclusions re-read on 14 November 2026.
- Merge-sequence deviation: **high for the recorded facts** (timestamps, account, merge commits, absence of review comments/reviews), **medium for attribution** (the API cannot distinguish human from agent use of the account; Owner confirmation pending).

## 11. Redaction (§15)

This record contains references only: repository, PR, run, job, comment and card identifiers, commit SHAs and timestamps. No credential value, token, private key, session cookie or personal secret appears in this file or in the commands above.

## 12. What this record does not do

- It does not change any branch-protection setting, required check, review mechanism or protection read-back.
- It does not flip any `CONTRIBUTING.md` row other than 116 and 117, and it does not tick acceptance box 3 (superseded, never PASS, D50).
- It does not claim a D39 verdict existed before either rev-1.9 merge.
- It does not qualify deletion, signature, update-branch, UI or content-current behaviour.
