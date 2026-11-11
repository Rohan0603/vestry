# Vestry design revision 5 — Chief disposition integration record

**Evidence record for D40 (baseline revision 1.6) and the Gate-2 HLD/LLD revision-5 drafts.**
This is a design-integration and governance record, not a runtime qualification. No product code
exists, no product test was run, and no paid provider call was made.

- **Date:** 12 November 2026
- **Run:** `chief-9aefaf60-stack-20261111` (Chief: Qoder; integration step dated 12 November 2026)
- **Machine:** Windows x64
- **Node:** v24.19.0 (CI pins 24) · **Python:** 3.14.7 local (CI pins 3.12)
- **Authorization:** Owner "Publish scoped corrections" (11 November 2026, issue #9 comment
  6803506170) — feature-branch commits, pushes, PR updates and deliberate merges after required
  checks and valid review; no direct-main or force pushes. D39 solo-review authorization (issue #9).
- **Baseline:** `REQUIREMENTS.md` revision 1.6, D1–D40; source `main` at
  `3cc45154325cce98b02f0e8cc34f3e256640cf78` (revision 1.5); branch `ves/8-design-rev5`.
- **Credentials used:** none · **Tokens spent:** none

No secret values are recorded in this file, per §15.

---

## 1. Scope and method

The Chief integrated the fourteen open design-issue dispositions and the #17 integrity-audit
cadence into the Gate-2 HLD/LLD drafts as revision 5. These are **D34 delegated technical
selections inside the already-approved envelope** — design choices, not new requirements — so they
require no Owner decision to record. The method was: read each disposition comment in full, map it
to the responsible design section, edit the draft to reflect the decided disposition, keep every
runtime control marked `target` (D32), and record the resulting approval state honestly. Two
baseline-level facts required a recorded change-control entry (D40) rather than silent drift: the
revision-4 conditional Gate-2 approval bound exact digests that the changed bytes no longer match,
and the live required-check set differs from D39's historical `governance` reference.

This record does not approve the design, qualify any runtime control, or authorize product
implementation. Fresh Owner Gate-2 approval of the revision-5 digests is **pending**.

## 2. Artefact digests and approval state

Digests are SHA-256 over the committed Git blob bytes (LF-normalized per `.gitattributes`
`* text=auto`), computed with `git show :docs/<file> | sha256sum` on the staged blob — the same
convention used for the revision-4 approval. Worktree CRLF conversion makes direct file hashes
unreliable; the blob hash is the bound artefact.

| Artefact | Revision | SHA-256 |
|---|---|---|
| `docs/HLD.md` | 5 (this record) | `e7ded45cdd545e4a95ec1763cacaf4d5152bcf4dbc582404c76788ff7ee7d009` |
| `docs/LLD.md` | 5 (this record) | `9a15d8806ea54e471d9fa76ae05ba80dcfdd42e2d090b0d51b874f5ff37394cc` |
| `docs/HLD.md` | 4 (conditional approval) | `9ebcfae21e7961e840f2f67d0811be137fe1819d037a9aa4a662eab42f55d3dc` |
| `docs/LLD.md` | 4 (conditional approval) | `27a3008df5620d3d5be22612152628aa485101fdba1634c8c4b5844372180e67` |

The revision-4 digests were re-verified from commit `6ce03c145462bcdb5d35a56075bae3235e834a95`
(`git show <commit>:docs/<file> | sha256sum`) and match the recorded conditional-approval values
exactly. **The revision-4 conditional Gate-2 approval bound those exact bytes (D30). Revision 5
changes the bytes, so that approval does not extend to revision 5. Fresh Owner Gate-2 approval of
the revision-5 digests above is pending, not granted, and is not assumed by this record.**

## 3. Disposition mapping

Fourteen design dispositions integrated as delegated selections; #17 is the integrity audit whose
five-minute R9 cadence was already aligned in LLD §14.

| Issue | Disposition subject | Design location |
|---|---|---|
| #14 | UI technology selection (embedded React/TS bundle, rust-embed) | HLD §3/§9; LLD §7; §18 item 7 |
| #18 | D26 canary inventory bound to dispatch fingerprint; bounded feasibility experiment | HLD §6; LLD §1 |
| #21 | Entrypoint discrimination test matrix (config/plugin origins, PURE, sentinels) | LLD §13 |
| #22 | Schema loss-impact/survivability supplement; FK-respecting migration order | LLD §3 |
| #23 | Telemetry exporter six-state model; bounded buffers; Langfuse storage deps | LLD §9 |
| #26 | Linear lost-create read-back; transport dedup; product connector retained | LLD §12 |
| #28 | Latency targets and retention; pinned benchmark environment | HLD §8; LLD §10 |
| #29 | Model pinning; four identities; disable fallback; generation read-back | HLD §6; LLD §8 |
| #30 | Enabling-path drift validation for all active object types | LLD §14 |
| #31 | Effect-gateway hard availability gate; daemon-owned typed intents | HLD §6; LLD §2 |
| #32 | Dispatch-readiness certificate bound to authoritative tables; mirror | LLD §11 |
| #33 | Compound clarification counts one reserved ask; unresolved park | LLD §6 |
| #34 | Run-bound spend warnings; unique usage-step sum; catalog prices | HLD §8; LLD §5 |
| #35 | Acceptance-check negatives (unknown executor, stale SHA, tampered evidence) | LLD §4 |
| #17 | Integrity-audit cadence (five-minute R9) — already aligned | LLD §14 |

The LLD security section (§7) was aligned to the Owner-decided D38 bootstrap-authentication
mechanism; it remains unproved, so protected mutation stays unavailable.

## 4. D40 change-control record and required-check reconciliation

D40 (baseline revision 1.6) records: (a) the fourteen dispositions as D34 delegated selections
adding no requirement; (b) the revision-4 approval bound old digests, revision-5 approval pending;
(c) the required-check reconciliation below; (d) the §18 item 7 delegated UI selection, item open
for Gate 2. No prior D-row was edited or deleted; D39's review mechanism is unchanged.

**Live branch-protection read-back** (`GET repos/mission-control-panel/vestry/branches/main/protection`,
12 November 2026):

- `required_status_checks.checks`: `Policy validator tests` (app_id 15368) and `Gitleaks secret scan`
  (app_id 15368).
- `strict`: true · `required_approving_review_count`: 0 · `dismiss_stale_reviews`: true ·
  `enforce_admins`: true · `required_conversation_resolution`: true · force-push/deletion blocked.

This reconciles D39's historical "required `governance` check" reference: the aggregate `governance`
job still exists in `.github/workflows/governance.yml` but is no longer the required context; the
required contexts are the two named checks above. The rename/split was already-applied authorized
CI work (VES-33), not a new protection change made by D40. The historical evidence file
`2026-11-07-vestry-governance-ci-required-check.md` records the earlier `governance`/approvals-1
state and is preserved unchanged; this record supersedes its protection facts in scope only.

## 5. Verification actually executed

Offline governance gates, run locally on branch `ves/8-design-rev5` against base
`3cc45154325cce98b02f0e8cc34f3e256640cf78`:

| Check | Command | Result |
|---|---|---|
| Validator compile | `python -m py_compile scripts/validate_governance.py scripts/test_validate_governance.py` | PASS |
| Python validator tests | `python -m unittest discover -s scripts -p 'test_*.py'` | PASS — 22 tests, 0 failures (local Python 3.14.7; CI pins 3.12) |
| Node checker tests | `node --test scripts/ci/governance-checks.test.mjs` | PASS — 15 tests, 0 fail |
| Document/decision checker | `GOVERNANCE_BASE_SHA=3cc4515… node scripts/ci/governance-checks.mjs` | PASS — 5 check groups; D1–D40 unique/ordered/contiguous; prior D1–D39 rows byte-preserved vs base |

The decision-row integrity check confirms D40 is additive: every prior D-row matches the base
exactly. The supplemental secret-pattern scan in the checker passed; this is a high-signal
guardrail, not a comprehensive secret detector.

## 6. NOT RUN and honest limitations

- **Gitleaks secret scan:** the required `Gitleaks secret scan` check runs in CI on the pull
  request; it was NOT RUN locally. The local supplemental pattern scan is not a substitute.
- **CI on GitHub Actions:** the required checks run on the PR after push; results are recorded on
  the PR, not claimed here in advance.
- **Runtime qualification:** NOT RUN. No Vestry product or harness exists; every runtime control in
  the drafts is `target` (D32). Canary enforcement, effect-gateway restriction, owned-process
  crash cleanup, served-model identity, spend measurement, latency targets and drift predicates
  are design dispositions, not demonstrated controls.
- **Product tests / paid provider calls:** NOT RUN. None were authorized or performed.
- **Gate-2 approval:** PENDING. The revision-5 digests require the Owner's fresh approval; the
  revision-4 conditional approval does not extend to changed bytes.
- **Python version qualifier:** local tests ran on 3.14.7; CI pins 3.12. The CI run is the
  authoritative gate result.
- The fourteen design issues remain **Blocked** on their runtime evidence; this integration does
  not close them.

## 7. Redaction and traceability

No credentials in any artefact; environment references are names only. Redaction check: PASS
(manual scan plus the checker's supplemental pattern scan). Traceability: D40 row and the
revision-1.6 change record in `REQUIREMENTS.md`; HLD/LLD revision-5 headers and LLD §15 history;
this evidence file. Publication proceeds only through a reviewed pull request under the Owner's
scoped-corrections authorization (issue #9), with independent non-authoring exact-SHA review (D39)
and a deliberate Owner-authorized merge. Refs: VES-8.
