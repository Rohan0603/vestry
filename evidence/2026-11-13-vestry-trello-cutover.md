# Vestry — Trello cutover and requirements revision 1.8 evidence record

**Date:** 13 November 2026
**Decision record:** D42, D43, D44, D45, D46 (REQUIREMENTS.md revision 1.8, §4)
**Status:** Migration complete and independently verified; requirements record landed; GitHub Projects cleanup authorized but **not executed**; no product implementation.
**Repository:** `mission-control-panel/vestry` at baseline commit `4f39cb1` (revision 1.7) when this record was authored.

This file is self-contained. It records the facts that D42–D46 rely on, the verification behind them, and what remains unproven. It does not depend on any working-directory or untracked file.

## 1. What changed, in one paragraph

On 12–13 November 2026 the Owner decided that Trello is the sole system of record for both this repository's task work and the Vestry product's project-management integration; that the implementing agent may merge its own reviewed pull request after the D39 independent exact-SHA review and green required checks; that Vestry will never use a paid model and that the §19 item 9 D20 re-proof is re-scoped to a pinned local open-weight model plus one pinned free-tier model; that the §20 domain, handle and crates.io actions are deferred until the project is complete and working; and that the organisation GitHub Projects v2 board may be deleted once the Projects-token dependency is removed from CI. These are recorded as D42–D46 in REQUIREMENTS.md revision 1.8. The requirements amendments cite their authorising D-row; no prior D-row was edited or deleted.

## 2. The board

| Item | Value |
|---|---|
| Board | Vestry — <https://trello.com/b/JiiGuQAi/vestry> |
| Source issues migrated | 36 scoped GitHub issues (25 open, 11 closed at migration time) |
| Cards on the board after migration | 36 |
| Comment count | 199 |

Card titles retain the GitHub issue number and the historical `VES-###` Linear identifier as provenance, for example `[Vestry #46] [VES-36] …`. The `VES-###` identifier is not a live reference: no Linear record is read or written (D42).

On 13 November 2026 a 37th card was created for this requirements/design/CI adaptation itself (short link `zxPBoy92`); the 36 above are the migrated issue cards, and the 36-card, 137-comment and 199-comment figures describe the migrated records only.

## 3. Comment fidelity — 137 of 137 byte-identical

Every GitHub comment body was migrated with a three-line layout: a provenance line `[GH @handle · UTC]`, a blank line, the GitHub body verbatim, then `[source: <html_url>]`. Bodies were compared byte-for-byte, not by normalised text.

| Check | Result | Basis |
|---|---|---|
| GitHub comment bodies byte-identical to source | **PASS — 137 / 137** | Direct byte comparison of source JSON body and migrated comment text |
| Total native comments on the board | 199 = 137 migrated bodies + 11 closure notes + 51 PR backlinks | Fresh board read-back, 199 unique comment ids, 0 duplicates |
| Layout failures / marker mismatches | 0 / 0 | Round-2 verification |
| Comment additions or deletions | none | Activity feed unchanged in comment-add/remove terms; edits create no board actions |

During verification 12 migrated comments were found to differ by one trailing newline or by emphasis placement. All 12 were corrected **in place** on 12 November 2026 between 20:30:46Z and 20:32:46Z. In-place edits preserve the original `createdAt` and stamp `lastEditedAt`; they do not create new board actions, so the action feed's newest comment entry remained 20:01:56.956Z throughout. The 12 corrections were re-read and confirmed byte-identical. This is recorded rather than hidden: comment fidelity is 137/137 **after** those 12 corrections, not as an untested claim of first-pass perfection.

## 4. Checklists and list state

Task-list items in migrated bodies were converted to native Trello checklists so they remain actionable rather than being left as static markdown.

| Measure | Value |
|---|---|
| Native checklists | 32 |
| Checklist items | 149 |
| Items checked | 29 |
| List distribution | `Done` 11, `Blocked` 23, `In Review` 2 |

The 11 `Done` cards correspond to the 11 GitHub issues that were closed at migration time; no open issue was marked Done. The distribution covers all 36 cards exactly once (11 + 23 + 2 = 36).

## 5. GitHub was not modified

| Snapshot | SHA-256 at migration time | SHA-256 at round-2 verification | Equal |
|---|---|---|---|
| Scoped issues | `2d59226dab072a587ee2e7b1248e4ba34efd00304c5e59bc8d38dc2dfc933da7` | same | **yes** |
| Issue comments | `125ff763623c30e0d20bdf94f87879c4a4fe0303ade906191ae51238106d396c` | same | **yes** |

These digests are over the serialised `gh api` pulls used for comparison (all repo issues, and all issue comments). Within the migration scope there were **0 issue diffs**, no comment edits, no closures and no new activity attributable to the migration. The read-back was read-only. No GitHub issue, pull request or label was deleted, and no GitHub Project was deleted by this work.

## 6. Independent verification

The migration's round-2 verification was produced by a verifier instance distinct from the migrating agent, using fresh live reads rather than the migration's own outputs: fresh board activity pages, fresh `gh api` pulls and fresh checklist reads on all 36 cards. It reported **no failures**. Its checks covered card count, per-card comment counts, byte-identical bodies, layout, duplicate markers, checklist counts, list distribution and GitHub immutability, and its totals reconcile with the migration report.

Verification strength, stated honestly: this is instance-independent verification of a data migration against live reads. It is not a proof that Trello's API will behave identically for the product at runtime, and it does not exercise any Vestry code — there is none.

## 7. Residual exceptions

| Ref | Exception | State |
|---|---|---|
| L1 | Native Trello labels were not present on the board (default unnamed labels, all unused); labels were represented as text in card content, verified 36/36. | **Open, documented.** No REST credential was provided for label administration. This is an optional enhancement, not a record-integrity gap: the taxonomy is present in text and checklists. |
| L2 | The board's cutover description was stale at migration time. | **Applied by the Owner on 13 November 2026** ("I've updated it"). The Owner applied it manually; an independent read-back of the new description text is **pending** and is not claimed here. |
| — | `VES-###` identifiers exist only inside card titles and comment provenance lines. | Accepted: historical provenance, not a live mapping. D42 makes Trello the record. |

## 8. Decision mapping

| Decision | Subject | Owner decision date | Authorization |
|---|---|---|---|
| D42 | Trello-only system of record for repository work and product PM integration; Linear and GitHub Projects removed as records | 13 November 2026 | Owner: "completely trello only, no Linear, no github project" |
| D43 | Standing self-merge by the implementing agent after D39 exact-SHA review and green required checks | 12 November 2026 | Owner: "if code review is done merge it yourself dont wait for me" |
| D44 | Never-paid model policy; hybrid re-scope of the §19 item 9 D20 re-proof | 12 November 2026 ("we will never use paid model") and 13 November 2026 ("hybrid") | Owner |
| D45 | Domain, handle and crates.io actions deferred until the project is complete and working | 12 November 2026 | Owner: "will do it at last after project completion and working" |
| D46 | Option-B GitHub Projects cleanup authorized, gated on the CI adaptation | 13 November 2026 | Owner: "B: delete the entire Portfolio project … all GitHub issues/PRs/labels stay intact; the legacy CI secret/variable is removed only if it exists" |

## 9. What this evidence does not establish

- **No product implementation.** No Vestry code exists; no product test, fault injection or provider call was run. Every runtime control remains `target` (D32).
- **No cleanup executed.** D46 authorizes deletion of the organisation Project v2 "Mission Control — Portfolio" (`PVT_kwDOFSJ1W84Bq4j0`, 170 items, 9 repositories) and conditional removal of the legacy `GH_PROJECTS_READ_TOKEN` secret / `GH_PROJECT_NODE_ID` variable. At the time of writing the board and any such credentials still exist; deletion is a separate, gated action whose own read-back must be recorded.
- **No Trello wire protocol.** The product's Trello integration — board and list structure, card schema, comment serialisation, idempotency keys and the `trello_card_id ↔ task_id ↔ attempt_id` binding — is revision-6 design work and is **pending fresh Owner Gate-2 approval** (D30, D41). The migration used the Owner-supplied connector; it is not a specification of the product's client.
- **No model qualification.** D44 pins the target models but proves nothing about them. Free-tier availability, local-model quality and tool-calling parity are milestone-1 work; unavailability yields `BLOCKED`, not a pass.
- **Label and description read-back.** L1 remains open; L2's owner-applied description has not been independently read back in this record.

## 10. Reproducing the checks

The round-2 verification compared (a) the set of 36 scoped issue numbers and their state, (b) every GitHub comment body against the migrated comment text with the provenance layout stripped, (c) the board's 199 native comments for uniqueness and per-card counts, (d) the 32 checklists / 149 items / 29 checked counts, and (e) the two GitHub snapshot digests above. Commands and outputs are held in the migration evidence set delivered with the migration; this record states the method and results so it can be re-derived from live state without that set.

*End of evidence record.*
