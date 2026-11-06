# Vestry

An autonomous engineering organisation in a box, for one person, on one machine, that cannot lie to you
about what it did.

Vestry is a durable local control plane. One developer states a goal once. A deterministic Chief then runs the
software delivery lifecycle — discovery, architecture, planning, decomposition, implementation, independent
review, verification with proofs, and closure — using Linear as its project-management record, spawning
specialised coding agents through a supervised adapter contract, and continuing without manual follow-up until
machine-checkable acceptance criteria pass and the Owner accepts.

It is a clean-room rebuild of an earlier system, and it is deliberately shaped by what that first attempt could
not prove.

## Status

**Requirements only. No code exists yet.**

| | |
|---|---|
| Baseline | [`REQUIREMENTS.md`](REQUIREMENTS.md), revision **1.2**, frozen as 1.0 on 6 November 2026 |
| Verification | Four dated evidence records under [`evidence/`](evidence/), all against OpenCode 1.19.3 |
| Project management | Linear — project **Vestry** (`P-VES-9`), team `VES`, issues `VES-###` |
| Agent instructions | [`AGENTS.md`](AGENTS.md) |
| Contribution rules | [`CONTRIBUTING.md`](CONTRIBUTING.md) |
| Next phase | Architecture design, starting from baseline §18 items 1 and 2 |

## Shape

One Rust binary. A headless daemon bound to `127.0.0.1` serving a local web UI. SQLite authoritative for
runtime state, WAL mode, one logical writer, transactional outbox. External coding agents supervised as **N
independent processes**, one per worker, each inside a Win32 Job Object on Windows or a process group on Unix,
each working in its own leased Git worktree at a recorded base SHA. No Electron, no native modules, no rebuild
against a runtime ABI — installation is copying an executable.

Three human gates — charter, architecture, plan — and then free run. Only actions outside the pre-authorised
envelope, and final acceptance, interrupt execution.

## What makes it different from "an agent that codes"

- **Truthful status beats optimistic status.** Unknown is a state, and it is displayed. A worker's own success
  claim stops at verification and never becomes acceptance.
- **Termination is proved, not acknowledged.** Killing a worker process is insufficient by construction: a hard
  kill was reproduced orphaning its descendants on the target machine. Vestry verifies its own process tree,
  before and after, and does not accept an adapter's "aborted" as evidence.
- **Permissions are proven in force, not merely set.** The worker runtime's built-in default is
  allow-everything, and a malformed policy degrades to it silently. Every dispatch therefore runs a policy
  canary.
- **Nothing is reported complete without evidence,** and evidence carries its own confidence, its limitations,
  and a three-state `PASS` / `FAIL` / `NOT RUN` rather than a boolean.
- **Drift is a computed graph property.** Every piece of work must cite the acceptance criterion it advances.
  Activity that cites none is parked; a criterion nobody is covering is an alarm.

## Local only

Single user, loopback binding, no remote access, no multi-tenancy, and no public-service behaviour. The daemon
and UI start and are fully usable with no provider configured, no worker runtime present, and no network.

## Licence

MIT — see [`LICENSE`](LICENSE).
