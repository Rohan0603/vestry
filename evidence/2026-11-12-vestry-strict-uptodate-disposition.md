# Vestry VES-4 — strict up-to-date and approval-dismissal disposition

**Evidence record for VES-4 (status-check / strict-up-to-date gate) and the D39 review model.**
This records what was actually demonstrated, what was not, and why — including an unintended
demo merge and its cleanup. No product runtime is involved; this is repository governance.

- **Date:** 12 November 2026
- **Run:** `chief-9aefaf60-stack-20261111` (Chief: Qoder)
- **Issues:** VES-4 · related VES-33 (governance CI), D39 (solo review), D32 (target/verified-enforced)
- **Authorization:** Owner "Publish scoped corrections" (issue #9) and D39 solo-review authorization.
- **Credentials used:** none · **Tokens spent:** none

No secret values are recorded in this file, per §15.

---

## 1. Configuration read-back (verified-enforced as configuration)

`GET repos/mission-control-panel/vestry/branches/main/protection` (12 November 2026):
`required_status_checks.strict: true`; required checks `Policy validator tests` + `Gitleaks secret
scan` (app 15368); `required_approving_review_count: 0`; `dismiss_stale_reviews: true`;
`enforce_admins: true`; `required_conversation_resolution: true`; force-push/deletion blocked.
The strict setting is configured and read back. Configuration presence is not the same as a
demonstrated isolated behavioral enforcement; that distinction is kept below (D32).

## 2. Strict up-to-date — demonstration attempt and empirical finding

A disposable PR #48 was opened to demonstrate the strict up-to-date gate. Design: branch
`ves4-demo-strict` was created from the previous main `3cc45154325cce98b02f0e8cc34f3e256640cf78`,
then its tracked content was brought up to the current main `63f256c691ae747a2d318d31223123c6cbab8073`
(so the required checks would pass), plus a harmless `demo-strict-behind.md`. The intent was a
branch whose **history** is behind main but whose **content** matches main, to isolate strict
up-to-date as the sole blocking gate.

Observed facts:

- Demo head `fec119b1ffd1b6def7049cbc9cee8aaf99b2a4f9`, parent `3cc4515`.
- `git merge-base --is-ancestor 63f256c fec119b` → **false**: main's latest merge commit was NOT an
  ancestor of the demo head; the branch was behind by commit ancestry.
- Required checks on the demo head: `Policy validator tests` SUCCESS (run 43092951013), `Gitleaks
  secret scan` SUCCESS (run 43092951028). Approving-count 0 and no unresolved conversations, so the
  review/conversation gates were satisfied.
- GitHub API: `mergeable: true`, `mergeable_state: clean`, `rebaseable: false`. The PR UI
  `mergeStateStatus` read `BEHIND` while checks were queued, then `CLEAN` once checks passed.
- `gh pr merge 48 --squash` (no `--admin`) → **exit 0, MERGED**. Main advanced
  `63f256c` → `1d940f446107c8d88f0943f4121f2765fe1cbd01`.

**Finding (empirical):** strict up-to-date did **not** block this merge. Although the branch was
behind by ancestry, its tree content already matched main, and GitHub reported `mergeable_state:
clean` and allowed the squash merge. The likely explanation is that GitHub treats a content-current
branch as up-to-date for this gate; this is stated as the observed behavior and a hypothesis about
the mechanism, not a certified internal claim.

**Consequence:** the demo file `demo-strict-behind.md` landed on main (`1d940f4`) unintentionally.
The net diff `63f256c..1d940f4` was exactly that one file (+7 lines); the revision-5 HLD/LLD digests
were verified unchanged (HLD `e7ded45c…`, LLD `9a15d880…`). The file is removed by the cleanup pull
request that carries this record; the disposable demo branch was deleted (GitHub auto-deleted the
remote head on merge).

## 3. What the demonstration did and did not establish

To make GitHub report a branch as behind and block the merge on strict up-to-date, the branch must
be behind in **content**, not only ancestry (§2 showed a content-current branch merges). The
governance checker (`governance-checks.mjs`) enforces **decision-row immutability** (plus markdown
hygiene, required files and evidence naming) against the PR base SHA — not full content parity. A
branch missing main's current `REQUIREMENTS.md` D40 row therefore fails the required `Policy
validator tests` check, so a merge rejection of such a branch cannot isolate the strict gate from
the status-check gate.

What was **not** tested: a branch behind only in *non-decision* content while keeping the current
D1–D40 rows (which would pass the checker). Whether such a branch would be reported as behind and
blocked by strict up-to-date in isolation is unresolved. An earlier draft of this record called
isolation "not achievable"; that over-generalized (independent review F3, comment 6806227452) — the
checker does not require full content parity, so achievability is not established either way.

**Disposition:** isolated strict-up-to-date behavioral enforcement is **NOT DEMONSTRATED**. The
setting is configured and read back (§1); the content-current merge behavior is recorded as an
empirical observation (§2); the non-decision-content-behind case is untested. This is an honest
limitation of what was demonstrated — not a claim that the gate is absent or misconfigured, and not
a claim that isolation is impossible.

## 4. Stale-approval dismissal — superseded by D39

The original VES-4 stale-approval-dismissal demonstration assumed GitHub approving reviews. Under
D39 the `required_approving_review_count` is 0: the sole collaborator is the commit-author identity
and GitHub forbids self-approval, so no GitHub approving reviews exist to dismiss. Independent
review is recorded as exact-SHA PR comments by non-authoring instances (D39), not native approvals.
**Disposition:** the stale-approval-dismissal demonstration is **superseded by D39**, recorded as
such — not marked PASS. `dismiss_stale_reviews: true` remains configured (§1) but has no native
approving reviews to act on in this review model.

## 5. Status summary (D29 / D32)

- **PASS (configuration, verified-enforced):** strict up-to-date, required checks, approving-count 0,
  dismiss-stale, admin enforcement, conversation resolution, force-push/deletion blocks all read back.
- **PASS (empirical observation):** a content-current, ancestry-behind PR with passing required checks
  was squash-merged under strict=true (PR #48); GitHub reported `mergeable_state: clean`.
- **NOT DEMONSTRATED (isolated behavioral enforcement):** strict up-to-date blocking a content-behind
  branch with otherwise-satisfied gates. A D-row-behind branch fails the checker (non-isolatable); a
  non-decision-content-behind branch with current D-rows was not tested (§3).
- **SUPERSEDED:** stale-approval dismissal by D39 (§4); no native approving reviews exist.
- **Cleanup:** the unintended demo file is removed by the pull request carrying this record; main's
  design digests were verified intact throughout.

## 6. Redaction and traceability

No credentials in any artefact; the demo file was harmless prose, not a secret. Redaction check:
PASS (manual scan plus the checker's supplemental pattern scan). Traceability: VES-4; PR #48 (demo,
merged then cleaned up); the cleanup PR carries this record and the demo-file removal; D39/D32 and
the §1 protection read-back. The earlier `2026-11-07-vestry-governance-ci-required-check.md` records
the prior `governance`/approvals-1 state and its non-isolated failing-check demo; it is preserved
unchanged, and this record supersedes its protection facts in scope only. Refs: VES-4.
