# Vestry PR #1 — unreviewed merge attempt rejected by base-branch policy

- **Date:** 7 November 2026, 07:46 UTC
- **Decision key:** `vestry-ves4-unreviewed-merge-attempt-20261107`
- **Issue:** VES-4 · **Baseline authority:** §21 (D35), D32
- **Relationship:** supplements
  [`2026-11-07-vestry-main-branch-protection.md`](2026-11-07-vestry-main-branch-protection.md),
  whose test 3 recorded only the `BLOCKED` merge state and explicitly deferred the merge attempt.
  This record executes that attempt. The earlier record's text is unchanged and was true when written.

## Why the attempt was safe to make

Before attempting, the protection read-back had confirmed `required_approving_review_count: 1` and
`enforce_admins: true`, and PR #1 computed `mergeStateStatus: BLOCKED` with zero reviews. GitHub
enforces merge eligibility server-side, so the attempt could only succeed if the hosted enforcement
itself failed — the exact hypothesis under test. No `--admin` or `--auto` flag was used.

## Command and result (PASS)

```
gh pr merge 1 --repo Rohan0603/vestry --squash
X Pull request Rohan0603/vestry#1 is not mergeable: the base branch policy prohibits the merge.
To have the pull request merged after all the requirements have been met, add the `--auto` flag.
To use administrator privileges to immediately merge the pull request, add the `--admin` flag.
EXIT=1
```

Post-attempt read-back: `gh pr view 1 --json state,mergeStateStatus` →
`{"mergeStateStatus":"BLOCKED","state":"OPEN"}`. PR #1 head remains
`6ce03c145462bcdb5d35a56075bae3235e834a95`; `main` remains
`f4c83ee79c818e4ef103a3f22a68bf277620dd0a`. Nothing was merged.

## D29

- **PASS, high confidence:** an actual merge call against an unreviewed PR was rejected by the
  base-branch policy. This upgrades the earlier state-level observation to a demonstrated merge-time
  rejection, satisfying VES-4's acceptance criterion "a PR with no review cannot merge — record the
  merge attempt and its failure" literally.
- **NOT RUN:** merge with a satisfied review (no reviewer exists — structural blocker unchanged);
  `--admin` bypass (not attempted; `enforce_admins: true` is read-back-verified and the push tests
  already demonstrated admin-subject enforcement).

## Redaction and traceability

No credentials or sensitive payloads; output contains only public identifiers. Redaction check: PASS
(manual scan). Traceability: `Refs: VES-4` commit footer → PR #4 → this file; Linear VES-4 comment
carries the decision key.
