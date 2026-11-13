# Owner Gate-2 and Gate-3 approvals, permissions, and VES-4 box-3 disposition

Date: 14 November 2026 (Owner local time, UTC+08; platform timestamps below are UTC).
Recorded by the Full-stack Engineer (T29) under the §4 change-control rule. Baseline record: REQUIREMENTS.md revision 1.9, decisions D47–D50.

## Decisions recorded

| Decision | Subject | Decided by the Owner |
|---|---|---|
| D47 | Gate-2 approval of HLD/LLD revision 6 at the reviewed, merged digests | 13 November 2026, by merging PR #53 himself |
| D48 | Gate-3 approval of the milestone-1 plan as drafted (T28) | 14 November 2026 |
| D49 | Three bounded operational permissions — (a) free-tier account + local Ollama for the Phase C re-proof, (b) branch-protection demonstration window R1/R3/R4, (c) Trello write-probes P1–P4 | 14 November 2026 |
| D50 | VES-4 (#8) acceptance box 3, stale-approval dismissal, superseded by D39 — not pending, not PASS | 14 November 2026 |

## Sources

- **Owner decision record:** Trello card `GgTpbAhr` (project record under D42), comment `6af78d46e80202d9ae575fa4` (2026-11-13T22:20:54Z), recording decisions (1)–(5) as received and verified. The plan of record is the same card's comment `6af78d80c0ed64c5831a8ac3` (2026-11-13T22:21:52Z).
- **VES-4 disposition:** Trello card `eePm8vuW` (#8), comment `6af78d47bf2eee582dd6660c` (2026-11-13T22:20:55Z): box 3 confirmed superseded by D39, box not ticked, not PASS, `dismiss_stale_reviews` left configured.
- **Merge event:** PR #53, state `MERGED`, base `main`, reviewed head `b1d25a7bb5eee3572b00f25ee6ba025793bc3543`, merged by `Rohan0603` (Rohan Ponnanna K K, human account, not a bot), merged at 2026-11-13T22:15:00Z, merge commit `2c0abd19fbddb36f41ad209e6729650ec74b8fd4`.

## Digest verification at the merged bytes

SHA-256 over the committed, LF-normalized Git blobs (`git show <ref>:docs/<file>`); the working tree uses CRLF, so working-tree bytes are not the digest input.

| Artefact | Digest at `2c0abd1` | Presented for Gate 2 | Verdict |
|---|---|---|---|
| `docs/HLD.md` | `7c78a7b3c5899a44ab9935de58fb893b97326dbbe312eb9edc03f8a1a6dfcb97` | same | MATCH |
| `docs/LLD.md` | `3a8c2d364d17c3a132af220a6f5d457a3ca018da7dbbb413bcfb68525a0100a2` | same | MATCH |

Under D30 the approval binds these exact bytes. Neither design file is edited by this record, and any later edit produces new digests that require fresh Owner approval (D41 precedent).

## Required checks on the merge commit

- Agent Governance CI — run `43517711366`, conclusion `success`, head `2c0abd19fbddb36f41ad209e6729650ec74b8fd4`.
- Secret Scan — run `43517711567`, conclusion `success`, same head.

## Branch-protection read-back used by the D50 disposition

Read-only `gh api repos/mission-control-panel/vestry/branches/main/protection`, 14 November 2026:

- `required_pull_request_reviews.required_approving_review_count`: `0`
- `required_pull_request_reviews.dismiss_stale_reviews`: `true` (configured; inert with no GitHub approving reviews)
- `required_status_checks.strict`: `true`; contexts: `Policy validator tests`, `Gitleaks secret scan`
- `enforce_admins.enabled`: `true`; `allow_force_pushes.enabled`: `false`; `allow_deletions.enabled`: `false`

No protection setting was changed by this record or by the T29 work.

## Scope of the approvals

D47 is a design acceptance: it qualifies no runtime control — every control remains `target` (D32) — and it authorises no product implementation by itself. D48 authorises a slow milestone-1 start in the approved dependency order and waives no gate: a dated D27 readiness certification is still required before any dispatch, and every §19 item is still evidenced `PASS`/`FAIL`/`NOT RUN` with a reason (D29); the plan's estimate is advisory (D16). D49 is bounded to the named actions and is not a protection-change or paid-model authorization. D50 leaves box 3 unticked and affects no other VES-4 box.

## Redaction pass (§15)

Checked before commit: no credential value, token, key, private-key block, database content, session store or user data appears in this file; credentials created for Phase C are referenced only by provider and account class, never by value. Only SHAs, run identifiers, setting names and public decision text are recorded. The repository's secret-pattern check and the Gitleaks required check run over this file in CI.

## What this record does not do

It does not claim any runtime control verified-enforced, does not claim the revision-6 Trello schema qualified (the P1–P4 probes are authorised, not run), and does not record the R1/R3/R4 demonstrations as executed — R3's attempt rides the merge of the change that introduces this record, and all three report separately. It is a decision record, not a status log (D23).
