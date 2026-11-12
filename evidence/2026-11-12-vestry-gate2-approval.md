# Gate-2 approval — revision-5 HLD/LLD (VES-8, #12)

Date: 12 November 2026. Recorded by the Chief under the §4 change-control rule and the Owner's "Publish scoped corrections" authorization (issue #9, comment 6803506170). Baseline record: REQUIREMENTS.md revision 1.7, decision D41.

## Decision

The Owner granted fresh Gate-2 approval of the revision-5 high-level and low-level design. In the Chief's session the Owner was presented the exact published revision-5 digests against baseline 1.6 (D1–D40) and answered **"Approve rev 5"** on 12 November 2026. This is an Owner design decision recorded by the Chief, not a Chief self-approval.

## Approved artefacts and exact digests

SHA-256 over the committed, LF-normalized Git blobs (`.gitattributes` `* text=auto`); the working tree uses CRLF, so digests are computed from `git show <ref>:docs/<file>`, not from the working-tree bytes.

- HLD revision 5 (`docs/HLD.md`): `e7ded45cdd545e4a95ec1763cacaf4d5152bcf4dbc582404c76788ff7ee7d009`
- LLD revision 5 (`docs/LLD.md`): `9a15d8806ea54e471d9fa76ae05ba80dcfdd42e2d090b0d51b874f5ff37394cc`

These bytes were published via PR #47 → merged `63f256c691ae747a2d318d31223123c6cbab8073` and are present on `main` at `ffc648711d25afbd4856cb78ea37cf340442aac6`. Under D30 the approval binds these exact digests. The HLD/LLD files are **not edited** by the approval record; their own status lines still describe the draft state as published, and REQUIREMENTS.md D41 is the authoritative approval of those exact bytes. Any later edit to either file produces new digests that require fresh Owner approval.

## Approval-state supersession

D40(b) recorded "fresh Owner Gate-2 approval is pending, not granted" because the revision-4 conditional approval bound different digests (HLD `9ebcfae21e7961e840f2f67d0811be137fe1819d037a9aa4a662eab42f55d3dc`, LLD `27a3008df5620d3d5be22612152628aa485101fdba1634c8c4b5844372180e67`, commit `6ce03c1`) that the changed revision-5 bytes no longer match. D41 supersedes that pending-approval fact in scope only; D40's row is unchanged. The fourteen §18 design dispositions integrated by D40, including the §18 item 7 delegated UI selection, are thereby Owner-ratified as the approved Gate-2 design.

## What this approval does not do

- It does not qualify any runtime control. Every control remains `target` (D32); no product code exists, and no product test, fault injection or paid provider call was run.
- It does not authorize product implementation or Worker dispatch. Those remain Worker scope requiring Gate 3 (plan approval) and a current D27 readiness certification. The §19 runtime proofs (#15, #19, #20, #24, #25, #27, #38) stay NOT RUN until a product exists to qualify.
- It does not close the design issues (#14, #18, #21, #22, #23, #26, #28, #29, #30, #31, #32, #33, #34, #35); their design is now Owner-approved, but they remain open for downstream runtime proof.
- It does not change any branch-protection setting, required check or review mechanism. D39 (instance-independent review, approving-count 0, Owner-authorized Tier-C merge), strict up-to-date, admin enforcement and the force-push/deletion blocks are unchanged.
- It does not extend the revision-4 conditional approval to changed bytes, and it does not widen the Chief's authority.

## Verification

- HLD/LLD digests re-computed from `origin/main` (`ffc6487`) before and after the approval record: unchanged at the values above.
- D1–D40 rows preserved byte-exact; D41 appended; governance decision-row integrity, markdown hygiene, evidence naming and secret scan checked offline against base `ffc648711d25afbd4856cb78ea37cf340442aac6`.
- Publication is through a reviewed pull request with an independent non-authoring D39 review bound to the exact head SHA and the required checks (Policy validator tests + Gitleaks secret scan) passing before a deliberate Owner-authorized squash merge. No direct-main, no force-push, no `--admin`, no secret in any commit, evidence, issue or log.
