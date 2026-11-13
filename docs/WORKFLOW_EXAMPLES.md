# Copy-ready card and comment examples

**Execution model:** the human owner manually invokes or schedules Codex, OpenCode, ChatGPT Work, or another
tool as a Chief/Worker session. The session reads the live Trello board and selects eligible work. Neither
Trello nor GitHub starts an AI model when a role token, card move or label changes. Use a unique per-session
claim and reconcile overlapping claims before doing work.

These are **examples only**. Replace card ids, short links, SHA values and dates. Never assert that a test ran
or approval happened because the example contains those words. The board is
<https://trello.com/b/JiiGuQAi/vestry> (D42); its lists are `Backlog`, `Ready`, `In Progress`, `Blocked`,
`In Review`, `Done`, `Canceled`, and `Needs Project Verification`.

## Session claim and release (advisory, not a lock)

Each separately started session checks **all card comments and live board state** before work (or explicitly
records the board-access gap in a metadata-bootstrap-only session). These example timestamps and IDs are
illustrative, not proof of a real agent run.

```text
<!-- claim:CARD-6af5a1d2a86a7094d8f28a13:codex-20261113T090000Z-1 -->
START — 2026-11-13T09:00:00Z
Role: worker; tool: Codex; run_id: codex-20261113T090000Z-1
Expires: 2026-11-13T10:00:00Z
Baseline: <actual-sha>; intended work: <approved scope>.
```

Read the START back, re-fetch claims and card list, and **stop without moving the card** if a different
active or ambiguous claim appears. Even two sessions using this sequence do not have an atomic lock.

At actual completion, escalation or deliberate stop, publish and verify the evidence/handoff **before**
publishing:

```text
<!-- claim-end:CARD-6af5a1d2a86a7094d8f28a13:codex-20261113T090000Z-1 -->
CLAIM RELEASED — 2026-11-13T09:40:00Z
Run ID: codex-20261113T090000Z-1
Outcome: <verified handoff / blocked / closeout URL>
Next owner: <chief / worker / none>
```

A claim with missing release or uncertain expiry needs Chief reconciliation. Never infer successful
completion merely from elapsed time.

## 0. First handoff: Trello metadata setup (required on every new card)

Chief includes this section even when the metadata is already complete. It is an **example**, not actual
evidence:

```text
TRELLO METADATA SETUP — FIRST HANDOFF
Board: Vestry, https://trello.com/b/JiiGuQAi/vestry (board id <board-object-id>)
Card: <card id> / <short link>; list: Backlog
Role text: worker VERIFIED / TO SET; Priority: P2 Medium — selected by Chief, VERIFIED / TO SET
Risk: Normal; Type: Implementation
Dependencies: none / blocked by <card id or url>, and the blocking card's list is Done VERIFIED / NOT VERIFIED
Linked PR/decision: <url> / none
Missing actions: none / list each operation to be completed by authorized Worker
Mode: Normal implementation handoff ONLY if full DoR and the card is read back in Ready.
```

### Bootstrap-only alternative when Chief cannot write the card metadata

A card is never treated as implementation-ready merely because its role text says `worker`. If Chief cannot
write or verify the card metadata but knows the **approved** target values, Chief records:

```text
<!-- metadata-handoff:CARD-6af5a1d2a86a7094d8f28a20-chief-001 -->
METADATA BOOTSTRAP HANDOFF — CARD <card id>
Role: worker; scope: METADATA ONLY; IMPLEMENTATION NOT AUTHORIZED
Board: Vestry, https://trello.com/b/JiiGuQAi/vestry; target list: Backlog (not Ready)
Priority: P2 Medium (Chief decision); Risk: Normal
Dependencies: <blocking card id/url> recorded as blocked by, or none
Current evidence: role text / list / priority / dependencies NOT VERIFIED (connector write 403 or missing scope)
To do: record the role text `worker` + `metadata-bootstrap`, place the card in Backlog, record Priority/Risk,
record the dependency references, and return each field's read-back evidence.
After evidence: worker -> chief; remove metadata-bootstrap; release claim.
Chief then performs the separate DoR and normal implementation handoff.
```

Chief changes the role text to `worker` + `metadata-bootstrap` **only** and reads it back. This is the **only**
exception to requiring a verified `Ready` card for a Worker *metadata task*. A **separately owner-invoked**
Worker may receive the card id directly or, in explicitly requested bootstrap queue mode, select cards carrying
both role tokens. Ordinary implementation queues must exclude `metadata-bootstrap` cards.

Worker confirms there is no conflicting claim, posts and reads back START, performs **only** the authorized
metadata writes and reads them back. The successful handback has **separate ordered steps**, not one
prematurely optimistic comment:

```text
<!-- metadata-prepared:CARD-6af5a1d2a86a7094d8f28a20-worker-001 -->
METADATA HANDOFF PREPARED — CARD <card id>
List: Backlog (read back); Priority: P2 (read back); Risk: Normal (read back)
Dependencies: blocked by <card id> (read back); role text: worker + metadata-bootstrap (read back)
Outstanding/NOT VERIFIED: none / <each blocked or unknown field>
Next: update role text to chief, remove metadata-bootstrap, read back the card.
IMPLEMENTATION NOT AUTHORIZED; this comment is NOT a completed handback.
```

After posting and reading back `METADATA HANDOFF PREPARED`, Worker performs the **one description update**
from `worker + metadata-bootstrap` to `chief`, preserving unrelated classification text. Worker reads back the
entire card and verifies the primary role is exactly `chief` and the bootstrap flag absent. **Only then** may
it post and read back:

```text
<!-- metadata-verified:CARD-6af5a1d2a86a7094d8f28a20-worker-001 -->
METADATA VERIFIED — CARD <card id>
Prepared evidence: <exact prepared-comment id and verified values>
Handback role text: chief only (actual read-back)
Outstanding/NOT VERIFIED: none; Ready was NOT asserted.
No product implementation performed.
```

If any field is missing despite a successful role-text update, post and read `METADATA BLOCKED` instead with
the exact unresolved values. After a verified final comment, post and read the matching `CLAIM RELEASED`
marker. Missing metadata never silently authorizes code or a PR. If creating a new list or choosing another
board is required, Chief supplies the exact approved structure or escalates to the owner; Worker never
guesses.

### Bootstrap reconciliation after competing or interrupted work

Claims are advisory, not locks. If two Workers have active or ambiguous claims, both stop before metadata
writes and Chief reconciles the card. A clean audit or pending-bootstrap marker does not resolve that conflict
or authorize a write.

For every Trello mutation, record the handoff ID and read back the card (and the specific field) after the
write. If a move or description update times out, returns an incomplete response, or is interrupted, do not
repeat it blindly: re-query first, detect whether the list/field already holds the intended value, and retry
only the missing operation. A card found on an unexpected list or as an ambiguous duplicate is a blocker;
stop and escalate instead of picking one. Record successful fields separately from unresolved fields.

If the prepared comment, the `worker` → `chief` role-text update, bootstrap-flag removal, final handback
comment, or any read-back fails, do not publish `METADATA VERIFIED` or claim completed release. Re-fetch the
**actual** card, post `RECONCILIATION REQUIRED` with exact successful, failed and unknown writes when safe,
then stop and report to Chief/human. Never assert Chief ownership when `worker` text remains. Require fresh
verification before compensation. The offline repository check can flag missing files or edited decision rows,
but cannot prove claim exclusivity, API atomicity, retry safety or handback success; those remain **NOT
VERIFIED** until live read-back.

Only after Chief independently certifies the full Definition of Ready and reads back `worker` + the card in
`Ready` does the normal implementation sequence in examples A–D apply.

## A. Chief creates a new implementation card

**Trello card:** `<card id>` / `https://trello.com/c/<shortLink>` — `[worker] Add idempotent request replay guard`

**List:** `Backlog`; role text: `chief` until DoR passes.

**Outcome:** On repeated identical request IDs, return the persisted result without issuing another external
side effect. Supports concurrency control and retry safety.

**Baseline:** `main` observed SHA `<actual-sha>`; existing files: `<paths>`; API version `<version>`.

**Dependencies:** `<blocking card>` recorded as `blocked by` on both cards. Explicitly record its list and
read-back evidence before Ready.

**In scope:** storage-level uniqueness, read-before-write, restart recovery, unit/concurrency tests; no
provider change or new runtime permissions.

**Acceptance:**

- Given the same request ID and payload on retried calls, when second call arrives, then exactly one side
  effect is attempted and a stable result is returned.
- Given same ID but different payload, then fail safely with a specified conflict status.
- Given restart after a committed result, then replay returns the persisted result.
- Given a concurrent collision, then the losing worker returns/retries according to documented policy without
  double execution.

**Exact checks:** `<real-command>` (unit tests), `<real-command>` (integration), `<real-command>` (lint), CI
`<workflow-name>`. Record all PASS/FAIL/NOT RUN.

**Failure/rollback:** Protect existing migration path, require forward/backward schema compatibility and an
explicit backout plan. Risk `Elevated` if live data migration applies.

**Chief prepares handoff:**

```text
<!-- handoff-id:CARD-6af5a1d2a86a7094d8f28a20-chief-worker-001 -->
CHIEF RESPONSE/HANDOFF — 2026-11-13T12:00:00Z
DoR: PASS on card <card id>; <blocking card> Done verified by commit <sha>.
Decision: choose database unique constraint + recorded replay result.
Approved file scope: <paths>. Rejected: in-memory-only deduplication.
Worker instructions: implement Given/When/Then cases above and capture actual commands/CI.
Risk: Normal for fixture implementation; any production migration escalates.
Next action: change role text chief -> worker and move card to Ready; read back both.
```

Before Worker begins, a real read-back must show **only** role text `worker`, the card in `Ready`, and the
HANDOFF comment.

## B. Worker requests Chief assistance

```text
<!-- handoff-id:CARD-6af5a1d2a86a7094d8f28a20-worker-chief-002 -->
REQUEST FOR CHIEF — 2026-11-13T15:15:00Z
Current head: <commit-sha>, branch ves/request-replay.
Problem: recorded replay payload can contain sensitive account identifiers.
Evidence: failing negative test <path>, command <command>, exit 1.
Options: hash canonical payload with salt; store redacted fields; encrypt payload.
Decision needed: approved retention/trust boundary and compatibility of old records.
Blocked scope: persistence schema change. Other changes paused to avoid rework.
Next action: change role text worker -> chief; move card to Blocked; read back both.
```

Chief cannot silently invent approval. Research alternatives, record the source-backed choice and, if needed,
`chief`+`human` while the card is `Blocked`. When resolved, post an actionable Chief handoff, remove `human` if
present, change `chief` → `worker`, and restore the correct list.

## C. Human-only gate, with Chief managing the card

```text
HUMAN ACTION REQUIRED — 2026-11-13T16:00:00Z
Owner must approve deployment of migration <id> on exact PR head <sha>.
Why human: production irreversible data changes; owner release approval required.
Acceptance evidence: card comment with scope, head SHA and explicit approval.
Until then: chief + human; card in Blocked; no merge/deploy.
```

After the owner records an action, Chief verifies the head and evidence, posts `HUMAN RESOLVED`, **removes
`human` itself**, and completes the authorized workflow. The human never has to edit the card.

## D. Worker completion and Chief acceptance

```text
REVIEW HANDOFF — 2026-11-13T17:00:00Z
Card <card id>, PR #<n>, exact head <sha>, base main@<sha>.
Changed: <file paths>. Acceptance matrix: <table with proof URLs>.
Checks: <command 1> exit 0, <command 2> exit 0; CI <URL> success on <sha>.
NOT RUN: hardware integration; linked validation <card id> with owner.
Security/rollback: <specific impacts>. Remaining risk <...>.
Next action: role text worker -> chief; move card to In Review; independent D39 review.
```

Under D39 an independent reviewer instance inspects that exact head and posts its verdict as a PR comment bound
to the head. Chief reviews that evidence and, under D43, the implementing agent merges by squash once the
required checks `Policy validator tests` and `Gitleaks secret scan` are green — no per-merge Owner wait. Chief
may instead request changes (`chief` → `worker`). For elevated/critical risk, require owner approval on the
exact head. Post CLOSEOUT with the final merged SHA, move the card to `Done`, and verify the read-back.

## E. Decision recorded in a card, not a conversation

Research discussion: compare a SQLite-backed lease with a Trello-comment-only advisory claim. Chief and
specialists debate options in a GitHub Discussion or on the card. Chief summarizes the approved outcome in
decision row D<n> of `REQUIREMENTS.md`, links the source, records real sources/version dates, and creates the
`blocked by` implementation cards with DoR. No Worker executes a task directly from a discussion thread.
