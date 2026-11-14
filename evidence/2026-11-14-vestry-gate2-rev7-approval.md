# Owner Gate-2 approval of design revision 7 — D51

Date: 14 November 2026 (Owner local time, UTC+08; platform timestamps below are UTC).
Recorded by the Full-stack Engineer (T40) under the §4 change-control rule. Baseline record: REQUIREMENTS.md revision 1.10, decision D51.

## Decision recorded

| Decision | Subject | Decided by the Owner |
|---|---|---|
| D51 | Gate-2 approval of HLD/LLD revision 7 at their reviewed and merged digests, expressed by his own merge of PR #60 | 14 November 2026, by merging PR #60 himself (`2026-11-14T05:19:18Z`) |

The Owner's message was exactly "PR merged". Under the D47 pattern, the Owner's merge of the exact reviewed bytes is the approval act; no separate digest-approval message was given, and none is inferred here. The three LLD §8.9 policy questions were **not** answered by this approval.

## Merge event

Read with `gh pr view 60 --repo mission-control-panel/vestry` on 14 November 2026:

- PR #60, state `MERGED`, title "docs(design): revision 7 — never-paid model pinning (G1–G6) and Trello write-window fixes", base `main` at `9b41e8cf5b005749eed9ca74eb1426574b32c22c`.
- Reviewed head at merge: `4abdc6665f556631611ffdce408a6c52c625910e`.
- Merged by `Rohan0603` (Rohan Ponnanna K K, human account, not a bot) at `2026-11-14T05:19:18Z`; squash merge commit and new `main`: `3370bc0c688eb565b8942b9c74fed2ace1bfc842`.
- Merge mode: squash. The branch `ves/design-rev7-pinning` was deleted after merge.

**Tree identity.** `git rev-parse origin/main^{tree}` = `a800a0a704ba26e2dc2de075fe1e0ae87d4845fc`; `git rev-parse 4abdc6665f556631611ffdce408a6c52c625910e^{tree}` = the same value. The merged tree is identical to the reviewed head's tree, so the approved bytes landed exactly.

## Digest verification

SHA-256 over the committed, LF-normalized Git blobs (`git show <ref>:docs/<file>`); the working tree uses CRLF, so working-tree bytes are not the digest input.

| Artefact | Digest at reviewed head `4abdc66` | Digest at merged `3370bc0` | Presented for Gate 2 | Verdict |
|---|---|---|---|---|
| `docs/HLD.md` | `963a0e3a44c2d3f4bf8fdbb5fe0c2b67c5ac4ae84aef113e4a2e22a059a1c38f` | same | same | MATCH |
| `docs/LLD.md` | `8341f3129434d18e0907364dc02049b6538f28af99cc20d3311ff6c40b5f24a0` | same | same | MATCH |

Under D30 the approval binds these exact bytes. Neither design file is edited by this record or by the T40 change; any later edit produces new digests that require fresh Owner approval (D41/D47 precedent).

## Review sequence (verdict first)

All three verdicts were posted as pull-request comments under D39 and read back with `gh api`:

| Stage | Comment id | Bound head | Verdict |
|---|---|---|---|
| T37 independent review | `6867480067` | `3d070d82f1c54205ec6dfcf515b6ff9391dcc62a` | REQUEST CHANGES — 13/15 PASS; policy-neutrality and internal-consistency findings F1–F3 plus risk R1 |
| T38 rework disposition | `6867582434` | `4abdc6665f556631611ffdce408a6c52c625910e` | Disposition mapping F1/F2/F3/R1 to their fixes; no policy question decided |
| T39 fresh independent review | `6867745351` | `4abdc6665f556631611ffdce408a6c52c625910e` | APPROVE — 12/12 checklist PASS, no blocking or minor findings |

The T39 verdict (`2026-11-13T23:41:08Z`) preceded the Owner merge (`2026-11-14T05:19:18Z`); the sequence was verdict-first. The T37 verdict remains bound to the superseded head `3d070d8` and does not extend to the merged bytes. A digest pair reported at the rejected head (`963a0e3a…` / `9686f2c8…`) is void for approval; the LLD digest changed in the rework, the HLD digest did not.

## Required checks

- At the reviewed head `4abdc66`: Agent Governance CI run `43527530879` (Policy validator tests + governance) — `success`; Secret Scan run `43527530848` (Gitleaks secret scan) — `success`.
- On the merge commit `3370bc0`: Agent Governance CI run `43560961865` — `success`; Secret Scan run `43560961889` — `success`.

## Trello record (card #29, `ebwetvnD`)

Read live on 14 November 2026 with the Trello connector:

- Card "[Vestry #29] [VES-16] Design §18-8", list **Done**; acceptance checklist 4/4 `COMPLETE`.
- Closeout comment `6af7f03f8301dd18a179b0d3` (`2026-11-14T05:23:11Z`) records the merge, the tree identity, the recomputed digest pair, post-merge CI, the verdict-first sequence and the pending-questions status.
- Owner decision package comment `6af7ef0e32dd8cd94592b1fa` (`2026-11-14T05:18:06Z`) preceded the merge and presented the digest pair and the three policy questions with fail-closed defaults.

The GitHub issue #29 closeout is separate repository work and is not claimed by this record.

## Pending policy questions (unanswered)

LLD §8.9 reserves three questions to the Owner (D34). The Gate-2 approval did not answer them; none is answered, inferred or recorded here:

1. **Registry recording/promotion/demotion authority (§8.6).** Default until answered: no row is promoted under the design alone; every enforcement-status row stays `target`.
2. **Any D44 amendment allowing a separately pinned, Owner-approved availability spare (§8.5).** Default: exactly two targets — one local open-weight digest and one free-tier identifier; unavailability is `BLOCKED`, never routed around; any amendment requires a new D-row through §4 change control before design text may allow it.
3. **Product-wide free-tier data policy beyond synthetic qualification/re-proof (§8.7).** Default: synthetic-only to any provider; real repository content stays local-only.

An Owner answer would be recorded as a decision or a new D-row under §4, not as an edit to the design or to this file.

## Scope of the approval

D51 is a design acceptance, not a runtime qualification. Every control remains `target` (D32); no product code exists; no product test, fault injection, probe or model call was run; all LLD §8 tests are NOT RUN. It authorises no product implementation by itself. Phase B implementation proceeds under the Owner-approved D48 plan and still requires a current D27 readiness certification before any dispatch; no acceptance criterion is weakened (D31).

## Redaction pass (§15)

Checked before commit: no credential value, token, key, private-key block, database content, session store or user data appears in this file; no model account, provider credential or runtime configuration is referenced except by class. Only SHAs, digests, run identifiers, comment identifiers, list names and public decision text are recorded. The repository's secret-pattern check and the Gitleaks required check run over this file in CI.

## What this record does not do

It does not edit the approved design bytes, does not claim any runtime control verified-enforced, does not close GitHub issue #29 (separate work after this record merges), and does not answer the three pending questions. It is a decision record, not a status log (D23).
