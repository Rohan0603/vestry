# Vestry governance CI — required check established and enforcement demonstrated

- **Date:** 7 November 2026 (08:21–08:26 UTC)
- **Decision key:** `vestry-ves33-governance-ci-required-check-20261107`
- **Issues:** VES-33 (this work) · VES-4 (status-check gate) · **Baseline authority:** §21, D32
- **Authorization:** Owner directive "unblock it" (7 November 2026), executing step 3 of the
  recorded procedure in VES-4 comment `fa039764-68ea-45bf-9a61-a847f2d917aa`
  (`vestry-ves4-independent-review-and-strict-checks-20261107`): establish an actual CI check,
  configure its exact name as required on `main`, independently read back.

## What was built

- `.github/workflows/governance.yml` — triggers on every `pull_request` (any base, because the PR
  stack targets feature branches) and on pushes to `main`; `permissions: contents: read`; no secrets;
  ubuntu-latest, Node 24 via `actions/setup-node@v4`; job id `governance` is the check context.
- `scripts/ci/governance-checks.mjs` — zero-dependency checks over `git ls-files`: §21 hygiene files
  present; Markdown UTF-8 / no trailing whitespace / final newline; `evidence/` filename convention;
  high-signal secret-pattern scan (GitHub tokens, fine-grained PATs, `sk-` keys, AWS key ids,
  private-key blocks); `REQUIREMENTS.md` decision-row uniqueness and revision header.
- Landed as commit `5a2d7a8ecbcbb059080a9381bf12bd24b4d21311` on `ves/33-governance-ci` from `main`
  (`f4c83ee`), PR #5.

## Verification actually executed

### Local pre-push verification

`node scripts/ci/governance-checks.mjs` on the CI branch: PASS (18 tracked files, including the
script scanning itself — no pattern self-matches). Same script against each stacked head:
`6ce03c1` PASS, `1a21f52` PASS, `3aa06d3` FAIL (1 missing final newline), `0b22160` FAIL (3 missing
final newlines). **The check caught real defects on first use.** Fixed by whitespace-only commits,
not by weakening the check: `22b4bb04e63ca4262760b67580d1b1136a874c37` (VES-24 record) and
`d21a47949f74fd9b257239203c073c3dc5daaa8a` (two VES-4 records). Text otherwise byte-identical;
blob SHAs changed and the change is recorded here and in Linear.

### CI on GitHub Actions — all five PRs PASS

The CI branch was merged down the stack (`--no-ff`, no force-push) so the workflow exists on every
PR head. New heads and results:

| PR | New head | `governance` | Run |
|---|---|---|---|
| #5 | `5a2d7a8ecbcbb059080a9381bf12bd24b4d21311` | pass (7s) | 42355278015 |
| #1 | `3f8a083bf3f82bcd634446a780708e8f95b18af4` | pass (5s) | 42355331762 |
| #2 | `c2967748bf4a94a26bd7807c821523a1ee09ba57` | pass (6s) | 42355348498 |
| #3 | `adef6a8d208d5e64421ad655d972a6a41bb85286` | pass (7s) | 42355351773 |
| #4 | `b35363430fee8d49ff24bea8f268e28341a04843` | pass (6s) | 42355353707 |

Gate 2 binding is unaffected: the HLD/LLD document digests approved on 7 November are unchanged;
the cascade adds only CI files and merge commits. No reviews existed to invalidate (all PRs had zero).

### Required-check configuration and read-back

`PUT repos/Rohan0603/vestry/branches/main/protection` with
`required_status_checks: {strict: true, checks: [{context: "governance", app_id: 15368}]}`
(pinned to the GitHub Actions app), all other settings unchanged → 200. Independent `GET` read-back
confirmed: `contexts: ["governance"]`, `checks: [{app_id: 15368, context: "governance"}]`,
`strict: true`, `required_approving_review_count: 1`, `dismiss_stale_reviews: true`,
`enforce_admins: true`, force-push/deletion blocks unchanged.

### Enforcement demonstration — disposable PR #6 (PASS, with stated limitation)

1. Branch `ves33-demo-failing-check` from the CI branch + `demo-violation.md` containing a deliberate
   trailing space; commit `658591b82216f0a560b3652ecd308985cf9b174e`; PR #6 against `main`.
2. `gh pr checks 6 --watch --fail-fast` → `governance fail (7s)`, exit 1, run 42355407047.
   Failed-job log: `governance: FAIL (1 violation(s))` / `- demo-violation.md:1: trailing whitespace`
   / `##[error]Process completed with exit code 1.`
3. `gh pr merge 6 --squash` → `X Pull request Rohan0603/vestry#6 is not mergeable: the base branch
   policy prohibits the merge.`, exit 1; `mergeStateStatus: BLOCKED`. No `--admin`/`--auto` used.
4. Cleanup: PR #6 closed with an evidence comment, remote and local demo branches deleted
   (`git push origin --delete ves33-demo-failing-check` — a disposable ref, not `main`).
   `main` verified unchanged: `f4c83ee79c818e4ef103a3f22a68bf277620dd0a` (`git ls-remote`).

**Limitation, stated honestly:** the merge rejection cannot isolate the status-check gate from the
review gate — both were unsatisfied (zero reviews, failing check). What is demonstrated: the required
check is configured and read back, it runs on PRs, it fails on a real violation, and a PR with a
failing required check is blocked from merging. A check-only rejection (reviews satisfied) remains
undemonstrated until an independent reviewer exists.

## D29 summary

- **PASS (high confidence):** check established and passing on all five PRs; required-check
  configuration read back; failing-check PR blocked from merging; `main` unchanged throughout.
- **NOT RUN:** strict up-to-date merge rejection (`main` has not advanced via an authorized merge —
  needs the reviewer first); stale-approval dismissal (no independent reviewer exists);
  check-only merge rejection with satisfied reviews (same reason).
- **Confidence qualifier:** measured on GitHub-hosted Actions, public free-plan repository. The
  secret scan is high-signal pattern matching, not a comprehensive secret detector — it is a
  guardrail, not a guarantee.

## Redaction and traceability

No credentials in any artefact; the demo violation is a trailing space, not a secret. Redaction
check: PASS (manual scan). Traceability: `Refs: VES-4` commit on PR #4 publishes this file;
VES-33 carries the implementation (`Refs: VES-33`, PR #5); Linear comments on VES-33 and VES-4
carry this decision key.
