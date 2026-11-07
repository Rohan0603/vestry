# Contributing to Vestry

These are the repository rules. They are the same rules Vestry is being specified to enforce on the
repositories it drives, which is why they are written down before there is any code: a rule the project cannot
satisfy on its own repository is a rule that will not survive contact with a real one.

## Branches

| Branch | Role |
|---|---|
| `main` | The protected base. Accepted state only. |
| `ves/<issue-number>-<slug>` | One issue, one branch. e.g. `ves/14-adapter-contract-shape` |

- Never commit directly to `main`.
- Never force-push, never delete `main`, never merge without the review the risk tier requires.
- Rebase is allowed only on a branch that has never been pushed. Once a branch has a PR, add commits.
- Squash-merge is the default, so one issue produces one commit on `main`.

## Commits

[Conventional Commits](https://www.conventionalcommits.org/), with the Linear issue in a `Refs:` footer:

```
docs(requirements): add D33 goal-drift control and D35 repository governance

Records the immutable goal anchor, the two machine-computed drift signals (orphan
work and blind spots), and the bounded charter-amendment count. Also records that
the baseline and its evidence were untracked in git before this commit.

Refs: VES-3
```

Types in use: `feat`, `fix`, `docs`, `test`, `refactor`, `perf`, `chore`, `ci`, `revert`.

- **One logical change per commit.** A commit mixing a baseline edit with tooling is split.
- The subject line stays under about 72 characters and says *what changed*, the body says *why*.
- A `docs(requirements)` commit must name the decision numbers it adds or closes. Baseline edits that do not
  cite a decision number are wrong by definition — see §4 of the baseline.
- `BREAKING CHANGE:` in the footer for anything that supersedes an approved decision in a way that invalidates
  downstream readiness certifications.

## Pull requests

Every PR states, in this order:

1. What changed, mapped to the acceptance criteria it satisfies.
2. The branch and the exact commit SHA tested.
3. Environment, exact commands run, results and exit codes.
4. Which checks were **not** run, and why. `NOT RUN` is a valid state; a silent omission is not.
5. Permissions, secret and licensing implications.
6. Known limitations, the rollback path, and any linked follow-up issues opened.
7. The risk tier, **reassessed against the actual diff** — not carried over from what was assumed at planning
   time. An upward reclassification re-routes the approval to the higher tier.

Review binds to the exact SHA reviewed. Any commit pushed after a review invalidates it, and the review is
re-obtained rather than extended.

**Solo-repository review model (D39, revision 1.5).** This repository has one Owner, who is also the
commit-author identity of agent-produced work, so GitHub cannot record a non-author approving review.
The §6 Phase 4 step 6 reviewer is therefore an independent reviewer *instance* — a subagent distinct from
the authoring agent — whose verdict and findings are posted as a pull-request comment bound to the exact
head SHA. The merge itself remains an Owner-authorized Tier-C action; a review comment is procedural
evidence, not a mechanical gate, and never substitutes for that authorization.

## What never gets committed

`.env*` and any credential in any form · SQLite or other database files · `auth.json` and worker session stores ·
generated output (`target/`, `dist/`, `out/`, `*.tsbuildinfo`) · `node_modules/` · runtime logs · evidence
that has not passed the redaction check.

A secret in a pushed commit is an incident, not a cleanup task: history rewrite on a shared host is not
reliably reversible. Escalate immediately and stop the push.

When you add a new class of generated artefact, add it to `.gitignore` **in the same commit** and say so in
the body.

## Evidence files

Everything under `evidence/` is a dated record of something that was actually done, and is treated as
immutable history rather than documentation to be tidied:

- One file per verification activity, named `YYYY-MM-DD-<subject>.md`.
- New trial, new path. Existing raw records are never overwritten.
- Each record states its environment, the exact commands, what was measured, **what was not measured**, and
  the confidence of its conclusions.
- Where a later record contradicts an earlier one, the later one wins and the earlier one gains an erratum
  banner pointing at it. The contradiction stays visible.

## Branch protection

`main` carries the following settings, applied 7 November 2026 under Owner authorization and verified
by API read-back plus live rejection tests
([`evidence/2026-11-07-vestry-main-branch-protection.md`](evidence/2026-11-07-vestry-main-branch-protection.md)),
adjusted the same day under D39 (approving-review count 1 → 0; review is the recorded independent-instance
verdict above).
Per the baseline's D32, a setting is `verified-enforced` only where enforcement was actually observed;
configured-but-undemonstrated settings stay `target`:

| Setting | Status | Basis |
|---|---|---|
| Require a PR before merging | `verified-enforced` | Direct fast-forward push to `main` rejected (GH006), as admin |
| Independent exact-SHA review (D39) | process-bound | Reviewer instance ≠ authoring instance; verdict recorded as a PR comment bound to the head SHA; merge is Owner-authorized Tier C. GitHub `required_approving_review_count` is 0 — the former `1` setting was removed 7 November 2026 under D39; its enforcement evidence (merge attempt rejected) remains in the dated records |
| Enforce on admins | `verified-enforced` | Both push rejections were issued to the admin account |
| Block force pushes | `verified-enforced` | Non-fast-forward force push to `main` rejected (GH006) |
| Dismiss stale approvals on new commits | `target` (moot under D39) | Configured; no GitHub approving reviews exist in the solo model. The equivalent rule — any post-review commit invalidates the review — binds the D39 review record |
| Require the `governance` CI check | `verified-enforced` | Check runs on all PRs; deliberate violation failed (run 42355407047) and merge was rejected ([evidence](evidence/2026-11-07-vestry-governance-ci-required-check.md)); review gate was also unsatisfied, so gates are not isolated |
| Strict up-to-date with `main` | `target` | Configured (`strict: true`); `main` has not advanced via an authorized merge, so the out-of-date block is undemonstrated |
| Block deletions | `target` | Configured and read back; deleting `main` was not attempted |

Applying or changing these settings needs repository administrator rights, which is an Owner action.
When a `target` setting is demonstrated, update this table in the same change that records the
evidence — never before.

## Before you open anything

Read [`AGENTS.md`](AGENTS.md). Its three "highest damage" rules — no in-place decision edits, no using the
baseline as a status log, no un-evidenced completion claims — account for most of the ways this project has
historically gone wrong.
