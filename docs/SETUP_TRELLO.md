# Vestry Trello board runbook

**Authority:** D42 (baseline revision 1.8) makes the Trello board **Vestry** the sole system of record for
this repository's task work and for the product's project-management integration. GitHub Projects and Linear
are not records. GitHub remains the code-review and CI host: pull requests, exact-SHA review comments, required
checks and the secret scan are unchanged.

Board: <https://trello.com/b/JiiGuQAi/vestry> — short link `JiiGuQAi`, board object id
`6af5a1d2a86a7094d8f289dd`, workspace `Mission Control`. The short link is a human-readable pointer; automated
writes key on the board, list, card and comment ids, never on display names, list positions or card order.

## 1. Board structure

The board was verified by read-back on 13 November 2026. It has eight lists and no custom fields:

| List | Purpose |
|---|---|
| Backlog | Recorded, not certified |
| Ready | Definition of Ready passed, routed to `worker` |
| In Progress | Claimed, actively executing |
| Blocked | Waiting on a dependency, decision or human gate |
| In Review | Implementation evidence published, awaiting Chief review |
| Done | Accepted, merged and proven |
| Canceled | Deliberately abandoned with a recorded rationale |
| Needs Project Verification | Quarantine for cards whose status could not be verified during migration; not a work queue |

A card's list **is** its status. Moving a card is a status transition; see the transition table in
[`GITHUB_AGENT_GOVERNANCE.md`](GITHUB_AGENT_GOVERNANCE.md) §3.

## 2. Role metadata

The board has six **unnamed** native labels with zero uses, and the connector cannot create or name labels.
The migration therefore preserved each source GitHub issue's label set as **text in the card description**,
verified 36/36 against the inventory. Roles are the exact lowercase tokens `chief`, `worker`, `human` and
`metadata-bootstrap`, read from that text.

Until an owner applies real labels in the Trello UI, treat the description text as the role record:

- a role change is proven by the handoff comment **and** the description text, never by a list move alone;
- a session that cannot write the description must not claim a role change — record `NOT VERIFIED` and
  escalate;
- priority, risk, type and dependencies (`blocked by <card id or url>`) are text too. Trello dependencies are
  advisory; Chief verifies the blocking card is `Done` before certifying Ready.

Applying native labels or renaming them is a manual owner action and is **not** performed by this repository.

## 3. Write discipline

- **Comment, don't describe, for history.** Card comments are the audit log. An in-place comment edit keeps
  the original `createdAt`, changes `lastEditedAt`, and does **not** create a new board action, so verify a
  comment by reading it back rather than by inspecting the activity feed.
- **Idempotency.** Put the `handoff-id` / `claim` marker in the comment text and search the card's comments for
  it before writing. After any ambiguous timeout, re-read the card and every requested field, then retry only
  the missing operation.
- **Expect a move, not a rename, for status.** Never infer status from a label or a title prefix; read the
  card's current list.
- **Never archive the record.** `Done` and `Canceled` cards stay visible; archival is not a closeout step.
- **Read back every mutation.** Record the actual values returned, not the intended ones. Partial success is
  reported as partial, with unresolved fields listed.

## 4. Rate limits

Verified against the Trello REST documentation on 13 November 2026: 300 requests per 10 seconds per API key,
100 requests per 10 seconds per token, and HTTP 429 when a limit is exceeded. Batch and paginate reads; on 429,
back off and re-read state instead of retrying blindly. Do not run parallel writers against the same card.

## 5. Credentials and CI

- Trello access uses an owner-held API key and token, supplied to the agent runtime outside the repository.
  No key, token or board-scoped credential is committed, logged or pasted into a card.
- The repository's GitHub Actions workflow runs **offline policy checks only** (decision-row integrity,
  required files, secret patterns, locked workflow assertions). It does not read the board and needs no
  Projects token.
- The legacy `GH_PROJECTS_READ_TOKEN` secret and `GH_PROJECT_NODE_ID` variable may be removed only under the
  gated authorization in D46: after this Trello-only adaptation merges to `main` with green required checks,
  and only if they exist. Repository issues, pull requests and labels are preserved.

## 6. Verified Trello endpoints used by the migration

These were exercised during the 13 November 2026 cutover and may be relied on for the same shapes:

- `POST /1/cards/{id}/actions/comments` — create a card comment (no client-supplied create key);
- `PUT /1/cards/{id}/actions/{idAction}/comments` — edit a card comment. This is the documented card-scoped edit route; the action-scoped `PUT /1/actions/{idAction}` is a different operation and is not used. Comment editing was exercised through the connector during the cutover, but the route has not been probed as a direct HTTP call in this repository, so its exact shape stays **NOT VERIFIED** until a write-probe reads the edit back;
- `GET /1/cards/{id}?actions=comments` — read a card with its comments.

Card `id` is stable across renames and list moves; the short link is not the identity. Other write endpoints
(card update, list move, checklist edits, label attach) were used through the connector and are documented by
Trello, but were not re-verified in this repository's check suite; treat them as **NOT VERIFIED** here until a
task reads their result back.

## 7. What this repository does not configure

- It does not create, delete or reorder board lists, and does not create or name labels.
- It does not configure Trello automation, Butler rules or member permissions.
- It does not start an agent from a card move, label or comment; the owner invokes or schedules each session.
- It does not delete the org Project v2 board; that is a separate, gated, owner-authorized action (D46).
