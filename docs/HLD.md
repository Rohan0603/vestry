# Vestry high-level design

Status: **Gate 2 draft; not approved or implementation-qualified**. Draft revision: 4, aligning the two Owner-decided baseline resolutions (D36/D37, baseline revision 1.3) after two independent advisory-review correction rounds. Date: 7 November 2026. Scope: VES-8 and VES-9–VES-22, Vestry only. Baseline: [REQUIREMENTS.md](../REQUIREMENTS.md), revision 1.3, D1–D37. Source checkout: `8b41f29` (revision 3 committed); this revision uncommitted on branch `ves/8-architecture-design`. Companion: [LLD.md](LLD.md).

“Decided” below means a delegated technical selection for this draft, not an Owner approval, a D-numbered amendment, or a demonstrated control. Every new runtime control is **target**. Historical D20 results keep their original scope and limitations. The baseline and original evidence are unchanged. No product code, installation, credential use, provider call, live process experiment, commit, push or PR is part of this design pass.

## 1. Purpose and boundaries

Vestry is a durable local engineering control plane for one Owner on one machine. Its deterministic Chief manages discovery, architecture, planning, execution, verification, independent review and acceptance. LLM agents provide judgement and implementation proposals; they never own lifecycle state, approvals or acceptance. [Baseline §1, §5–§9; D1–D10]

One Rust executable contains the daemon, bundled SQLite, native supervision and embedded browser assets. Browser TypeScript is a build-time asset language, **not a TypeScript backend or an installed runtime dependency**. OpenCode is a separately discovered and qualified external worker dependency. No v1 source or database is reused. [§14; D1–D4]

The daemon and browser experience work offline without Linear, a model or OpenCode. Missing dependencies disable affected dispatch capabilities, not history, evidence, policy-reference editing or Doctor views. No remote access, multi-tenancy, public service, nested workers, auto-merge or protected-branch bypass is introduced. [§14–§16]

## 2. System structure

```text
Owner browser on 127.0.0.1
  embedded UI -> typed local HTTP commands / replayable SSE
                         |
                 Rust daemon / Chief
    +--------------------+--------------------+
    | Gate/anchor/plan    | policy + effect gateway
    | scheduler/Doctor   | verifier/review coordinator
    | Linear connector   | adapter + owned-process supervisor
    +--------------------+--------------------+
                         |
        one logical SQLite writer / transactions
        audit + inbox + outbox + fencing + evidence metadata
                         |
    +--------------------+--------------------+
    | owned Git worktree | independent OpenCode process
    | per-attempt lease  | per-attempt adapter store
    +--------------------+--------------------+
                         |
    bounded-project Linear / explicitly authorised provider
```

Three active task slots are the default. Architect, Planner, Worker and Reviewer invocations all use one `HarnessAdapter` contract, with role-specific capabilities and admission records. They consume the same global process budget rather than spawning an uncounted planning pool. Architect/Planner admission binds authorized upstream scope and cannot require approval of the artefact that invocation will produce. Full thirteen-check D27 certification applies to Worker execution after Gate 3; canary, policy, ownership and capability checks apply to every role. Reviewer receives a fresh process, session and attempt and the exact candidate SHA. [§5–§8; D5–D7, D14, D27]

### Authority and trust

| Concern | Authority | Other representations |
|---|---|---|
| Scope, intent and project-management record | Bound Vestry Linear project; approved baseline and gate artefacts | SQLite stores approved revisions and provenance, not a competing editable backlog |
| Runtime, approvals, leases, ordering, recovery | SQLite | UI projections and Linear markers are read-backed representations |
| Accepted source and versioned artefacts | Git | SHA/hash bindings in SQLite and Linear; commit prose never authorises an effect |
| Worker conversation | Adapter session store | Recoverable handle only; not acceptance or approval truth |
| Governed action evidence | Native hash-chained audit | Optional telemetry is nonauthoritative |

Repository files, issue text, tool output, plugins and agent messages are untrusted data. Typed ingestion cannot elevate a role, approve an action, rewrite a criterion or authorize a network destination. [§10, §12, §15; D21, D31–D33]

Worktrees separate concurrent changes; they are not filesystem or credential sandboxes. Job Objects own ordinary descendant processes; they do not restrict network, file access, privileged handle manipulation, or work delegated through a system service/WMI broker. No design claim treats either as general hostile-code containment. [§11, §15–§16]

## 3. Reversible technology selections

| Selection | Rationale and rejected alternative | Qualification boundary |
|---|---|---|
| Rust + Tokio; native `windows` bindings | Existing D3. Tokio schedules I/O; a small Rust Win32 boundary performs creation-time Job assignment. Reject spawn-then-assign and C++ helper. | Windows 10+ `JOB_LIST` support; incompatible enclosing Jobs fail owned spawn. No invented Tokio process-attachment API. |
| `rusqlite` with bundled SQLite | D3/§12; one dedicated blocking writer thread, separate short read snapshots. Reject an ORM/service database/second state store. | Context7 inspected 0.41.1 documentation, not an installed dependency. Exact lockfile, SQLite build and MSRV checked before implementation. |
| Axum 0.8 family + typed JSON + SSE | Rust/Tokio ecosystem, browser-native unidirectional live state. Reject WebSocket until a real bidirectional requirement exists. | Context7 inspected 0.8.4 documentation. Replay and slow-client handling are Vestry responsibilities. |
| React 19 + TypeScript + Vite browser build | Eight interaction-heavy views, reusable accessible components and typed API projection. Adopt the VES-15 preliminary direction, not its unmeasured performance claims. | No runtime Node, CDN fonts/scripts, remote assets or LLM call to render. Exact compatible versions before build. |
| Tailwind 4 + selected shadcn/Radix components; TanStack Query/Virtual | Build-time styling; accessible interaction primitives; query cache and virtualized long histories. Reject importing every component or a second client-side authority. | Keyboard, focus, contrast, bundle and streaming tests mandatory; libraries do not prove accessibility. |
| Compile-time `rust-embed` asset manifest | Installation remains copying one executable; asset and API revision ship together. Reject loose required UI directory and runtime filesystem lookup. | Pin crate/version/licence at implementation; no automatic downloading or building at application startup. |
| Native audit + `tracing`; optional Rust OTel | D19/§12. No backend by default, metadata-only opt-in loopback export. Reject LangChain and all LangSmith deployment forms per baseline. | OTel trace APIs remain pre-stable in inspected 0.33 release documentation; pin compatible SDK/bridge versions together. |
| Raw Rust HTTP/SSE for optional private `serve` | Preserves Rust-only backend and explicit allowlisted external routes. Reject TypeScript SDK, sdk-next internal coupling and shared server. | `serve` is not enabled by the default design and requires separate P2a/P6 qualification. ACP has no initial role. |

These are selections under D34, with rationale recorded in the corresponding issues. They add no approved requirement and do not alter a D-numbered choice. Package versions named here are research snapshots/candidate families; none is silently “latest” or ready for dispatch.

## 4. Lifecycle and durable effects

Intake feeds one priority/dependency queue. Pre-anchor discovery binds explicit Owner intake/target/scratch scope without synthetic criteria; once Gate 1 freezes the anchor, all later admissions require current anchor/criterion edges and drift checks. The baseline's pre-anchor wording tension is resolved by D37 (revision 1.3): discovery-only dispatches bind the Owner's durable intake/target/scratch scope, and every dispatch from the frozen anchor onward falls under D33 without exception. Gate 1 approves the charter and immutable anchor. Gate 2 approves these HLD/LLD artefacts. Gate 3 approves the plan. The fixed thirteen-check readiness certification then authorizes only a specific task revision/base/environment; it is not another human gate. [§6; D8, D27, D37]

Execution obtains a fenced worktree lease, read-backs START, runs an owned worker, verifies objective checks, obtains independent exact-SHA review, applies bounded repair and synchronizes proofs/status. A verified task becomes `verified_handoff`, not merged/closed. Successors may use its immutable independently verified candidate; an execution lease may retire only after all owned processes exit and every change is safely recorded, while source/evidence refs remain retained. Integration consumes those candidates; Owner-authorized reviewed merge is independently observed before `integrated_closed` and merged-SHA CLOSEOUT. Closing children does not prove the parent. Acceptance requires every issue in the approved plan closed with independently valid attached proofs and confirmed remote closure, all original anchor criteria PASS, complete goal evidence, exact-SHA integrated independent review, no open orphan work or unresolved blind spot, and the Owner's acceptance. A covered blocked task still prevents acceptance; terminal withdrawal/cancellation is not proof of completed planned work. [§6, §9–§10; D10, D29–D33, D36]

**Baseline sequencing discrepancy — resolved by D36 (revision 1.3, Owner-decided 7 November 2026):** the former §6 Phase 4 close-before-integration wording is superseded. Execution ends in `verified_handoff` with proofs attached and source retained; closure and merged-SHA CLOSEOUT occur only after the Owner-authorized, independently reviewed Phase 5 merge is observed. The remote workflow mapping must still be conformed to this order in the implementation design; no closure capability may fabricate Done/CLOSEOUT before the observed merge, and the handoff/closure split is now baseline law rather than a pending disposition. [§6 Phase 4 step 9 / Phase 5; D36]

Every state-changing external effect has a transactionally committed intent, idempotency key, actor/sequence/fence and native audit event before execution. Result recording and corresponding messages are committed together. An uncertain remote outcome is reconciled through read-back; it is not blindly retried. No claim is made that SQLite and Git/Linear/Win32 share a physical atomic transaction. [§10–§12]

The ten-minute watchdog measures absence of meaningful durable progress. Raw token traffic, heartbeats and SSE keepalives do not reset it. Process liveness, output activity and known command deadlines inform diagnosis; they are recorded as observations, not invented progress. One diagnosis retry is bounded inside the attempt retry budget; repair cycles stay bounded at two per stable issue identity. Clarification questions, repair ordinals and diagnosis/retry reservations are durable, idempotently keyed and transactionally reserved before their effects; restart cannot reset them. Ordinary and watchdog retries retain the same attempt identity/count and increment dispatch ordinal/fence only after independent prior-tree termination and effect/source reconciliation; failed-candidate repairs consume the stable issue budget and use a new attempt. Uncertainty blocks replacement, never overlaps Jobs or silently replays a tool. Spend never stops work. [§8; D14–D16]

## 5. Supervision and recovery

### Windows owned spawn

Create a fresh unnamed non-inheritable Job per attempt dispatch, with at most one active dispatch/Job per attempt, set and read back `KILL_ON_JOB_CLOSE`, and leave breakaway disabled. Use `CreateProcessW`/`STARTUPINFOEXW` with `PROC_THREAD_ATTRIBUTE_JOB_LIST` and an explicit child stdio `HANDLE_LIST`, plus `CREATE_SUSPENDED`. Job membership is established **during creation**, before any initial-thread execution. Record root PID and exact creation FILETIME while suspended, then resume once. Closing the daemon's last Job handle after a hard kill triggers kernel cleanup. [§11; D3, D5; sources W1–W3]

Ordinary `tokio::process::Command::spawn` followed by assignment has an execution race. Suspended creation followed by assignment removes that execution race but still leaves an unowned suspended process if the daemon dies between creation and assignment. Neither is the fallback. Unsupported creation-time assignment fails closed. Windows raw-process ownership is an implementation of the portable Tokio-supervised process contract, not a second executable. [W1–W3]

Forced cancellation terminates only the owned Job. Before reporting cancellation complete, require root signalling plus complete empty Job process-list and zero active-process accounting. ACK, root exit, EOF and completion notifications are insufficient. Query failure or timeout yields `recovery_required`, never a clean cancellation claim. [§11; W3–W5]

On restart, the unnamed Job cannot be reopened. Stored PID/creation time identifies observed processes but cannot prove absence of unobserved descendants. Rely on the separately qualified kernel kill-on-close guarantee for cleanup; mark per-attempt proof unavailable where it is unavailable. No PID-tree guessing or killing unrelated processes to manufacture proof. A qualification observer may retain process handles, never a Job handle that would defeat kill-on-close. [§11–§12; W3–W5]

### Portable boundary

Unix uses a dedicated process group behind the same API. `setsid` can escape a group and groups do not provide Windows-style parent-crash cleanup. That backend cannot advertise all-descendant hard-crash cleanup merely because negative-PID signalling works. Initial milestone qualification is Windows-first; unsupported portable guarantees remain unsupported rather than changing D3 into an invented cgroup dependency. [§11, §14; W6]

### Restart order

Exclusive daemon generation and migrations precede readers/dispatch. Doctor reconciles launch intents, process identity, worktrees and dirty changes, unfinished checks/reviews, approval consumption and outbox outcomes. Unknown becomes `recovery_required`. Known completed effects may be recorded after independent observation; unknown effects are never replayed. Retain uncertain leases and user changes. A resumable OpenCode session is not permission to replay its interrupted action. [§8, §11–§12]

## 6. Adapter and policy feasibility

### Default adapter

The default is independent `opencode run --format json`, OpenCode **1.19.3**, explicit `--dir` and requested provider/model route, isolated Windows-absolute session store, no generated fresh `--session`, no auto aliases. Session ID is learned from actual output; resume requires that observed handle and Doctor reconciliation. Structured `step_finish` supplies per-step token/cost/snapshot metadata. CLI exit is not task acceptance. [D20; §7–§8, §11–§12]

Capabilities are `supported`, `unsupported`, `conditional` or `unverified`, each tied to a version/environment/evidence fingerprint. Unverified is unavailable for dispatch. Required capability failure blocks execution while the offline daemon remains usable. [§14, R5]

### Canary selection and limitation

Run D26 preflight on **every dispatch path**, including planning and review: validate and deterministically serialize policy, inspect effective config/agent/tool inventory, then run a bounded deny-all canary with the pinned model. PASS requires machine-observed registry/tool-list evidence for the relevant runtime, not a model saying it was denied or merely omitting a tool call. A canary that never provokes observable registry evidence is NOT RUN and blocks real work. [D26; §18 item 1; O1–O2]

`debug config` and `debug agent` are candidate inspection contracts, not proof of the final session-prepared tool list. They can themselves load config/auth and initiate dependency work. `debug agent --tool` executes a tool and has different approval semantics; it is explicitly not the canary. Inspection/model dispatch must run only inside an authorised, owned qualification profile. MCP/custom/plugin tool paths and model-dependent registry differences require separate conformance. No real worker starts with an unverified final-tool policy. [O1–O2]

### Exact grants are not static allowlists

A static `OPENCODE_PERMISSION` allow pattern has no expiry or consumption identity. Re-dispatching an approved pattern cannot honestly enforce one-use approvals. Private `serve` reply `once` resolves one permission request, not necessarily one complete invocation, and its payload has neither argument digest nor execution-time expiry. These paths **do not qualify exact-action, single-use, expiring governed effects**. [§7, §15; O3]

Design selection: Chief-owned typed effect execution for Tier B/C, with argument/target/SHA/policy binding, grant consumption immediately before a durable effect intent and no replay after an uncertain outcome. The LLM proposes; it never receives a reusable privileged shell pattern. Stop/fence relevant worker mutation and independently snapshot its target before such an effect. A later worker dispatch may continue ordinary authorized work, but cannot reuse the privileged grant. [§5, §7, §15; D21, D30]

This is a proposed implementation of the existing envelope, not permission to relax it. Until the restriction of effect-capable worker tools, egress and target mutation is demonstrated, that execution profile remains unavailable. Direct OpenCode shell allowance cannot be treated as a network/security boundary, including when its nominal command is a build script. The milestone is blocked on these proofs; enabling `--auto`, a plugin bypass or a shared server is not a workaround. [§7, §15–§16]

### Provider/model identity

Persist requested logical route, resolved OpenCode selection, configured API model alias, actual observed provider identity when available, and the evidence source separately. Export/message metadata proves OpenCode's resolved selection; it is populated before inference and does not attest the provider actually served that model. Opaque free/auto routing is ineligible for strict served-model qualification. Unsupported provider fidelity blocks that profile rather than silently claiming a match. [§8, §14, §16; VES-16, VES-23; O4]

## 7. D20 evidence reconciliation

| Evidence tension | Disposition |
|---|---|
| Source read followed V2 `Effect.ignore`; live V1 uses `Effect.orDie` | Preserve the erratum and live-path evidence. Still cooperative; hard kill does not run the cleanup chain. |
| Original source conclusion said JSON lacked usage | Live `step_finish` contains usage. Use that event; no separate completion event is invented. |
| Default allow versus headless auto-reject | Auto-reject applies only to requested permissions. Explicit validated policy and D26 proof are mandatory. |
| Baseline/live-probe narrative says `OPENCODE_PURE` reaches only TUI | New pinned-source inspection finds `plugin/index.ts:181` excludes external server plugin origins when `flags.pure`, wired from `OPENCODE_PURE` at `effect/runtime-flags.ts:18`. P1k measured bash allowance, which does not by itself establish plugin loading. This is a source-level contradiction, **not a new live proof**. Original records/baseline remain unchanged. VES-21/VES-9 must re-probe plugin-list suppression before adopting it as enforced mitigation; any risk/baseline amendment is Owner-reserved. |
| No hermetic config versus XDG relocation | XDG roots and auth-content controls are candidate isolation improvements from source, not a universal hermetic switch. Managed config, home discovery, custom tools, installed code and inherited credentials remain separate. No sandbox claim. |
| JSON-requested model versus actually served model | Export exposes resolved-selection metadata, not backend attestation. Strict capability stays conditional/unverified until provider-specific evidence. |

Source tag verified through GitHub: `v1.19.3`, commit `e3c4eb242585dcd77be1a867391b7c25fd47ce52`. New source findings are embedded here as design provenance; no raw D20 record is rewritten.

## 8. Concrete target proposals, not measurements

All figures below are **proposed acceptance targets under D25**, pending Gate 2 and milestone-1 measurement. They are neither inherited measured performance nor policy-envelope changes.

| Dimension | Proposal and measurement |
|---|---|
| Offline cold start | p95 ≤2 seconds to local UI usable state, ≥20 starts on recorded machine/build; missing providers/runtime must not prevent it |
| Dispatch readiness latency | p95 ≤5 seconds from already-certified queue selection to owned worker creation, excluding canary/provider latency, external authorization and reconciliation; report total cold first-dispatch latency separately, never hide exclusions |
| Idle daemon resources | private working set ≤100 MiB and mean CPU <1% of one core over ten idle minutes; record browser and workers separately |
| Active daemon resources | private working set ≤200 MiB while three workers each produce 100 KiB/s metadata/output; proposed budget: 100 MiB base + bounded 3×8 MiB queues + read/query/asset headroom |
| Whole system peak | **Not yet measurable**; report daemon + browser + three workers + descendants under named workloads. A provisional 2 GiB observational reference is not a worker memory kill limit. Qualification may require an explicit target disposition, not a fabricated PASS. |
| UI interaction | p95 ≤100 ms local filter/selection response with 50 issues and three streams; state-event-to-render p95 ≤250 ms, stale/disconnected state visible ≤5 seconds |
| Build assets | initial compressed browser JS/CSS ≤1 MiB; no remote assets; measure release bundle, not dependency marketing figures |
| Native database | proposed ≤100 MiB growth per 1,000 completed standard tasks with ≤20 steps/task and ≤2 KiB normalized audit metadata/event; report actual bytes/task and WAL separately |
| Adapter store | remeasure bytes/session/message under realistic workloads; historical 55 KB/session and 18 KB/message describe the **same sample**, not additive growth rates |
| Retention | all in-flight/recovery stores retained; terminal stores become deletion-eligible after redacted export, integrity/read-back and 7 days; no automatic deletion absent exact Tier-C approval/Owner policy resolution |

Warnings, not cutoffs: propose 100k/250k/500k tokens per invocation, 0.5M/1M/2M per goal and 1M/2M/4M per rolling 24 hours. Monetary equivalents require Owner-approved provider pricing and configured thresholds; free-router zero cost is not paid-cost proof. Missing usage is unknown, not zero. Selection rationale: the measured 20–24k input-token floor makes a 100k first warning roughly several uncached steps, while caches and workload variation require displaying the counters rather than predicting certainty. D16 has no circuit breaker. Actual threshold activation is an Owner policy setting. [§8; VES-13]

## 9. §18 disposition and traceability

Every row is **decided for the draft**, with detailed contracts/fault cases in the LLD. Formal baseline closure, Gate 2 and runtime qualification remain separate.

| Item / issue | Selection | Baseline |
|---|---|---|
| 1 / VES-9 | One capability-driven adapter; CLI default; raw Rust optional HTTP; every-path canary; exact privileged effects Chief-owned; unresolved proofs block profiles | §7–§8, §11, §14–§15; D4–D5, D20, D26 |
| 2 / VES-10 | Same adapter for every LLM role, role-specific permissions; planning has scratch/snapshot input, no writable target lease | §5–§6; D7, D21 |
| 3 / VES-11 | Single writer, append-only approvals/audit/anchors, fenced leases and transactional inbox/outbox; recoverable adapter handles | §10–§12; D3, D33 |
| 4 / VES-12 | Versioned typed executable-check manifest with immutable hash, tri-state outcomes; nonmechanizable criterion stops Gate 1 readiness | §6, §9; D29–D31 |
| 5 / VES-13 | Nonblocking multi-level token warnings; monetary levels only Owner configuration; deduplicated inbox/Linear/OS notification | §8, §13; D13, D16 |
| 6 / VES-14 | Three atomic Owner clarification questions per goal, one consolidated unresolved decision at a time; no compound-question budget bypass | §6, §8; D34 |
| 7 / VES-15 | Embedded React/TypeScript browser build; Axum REST/SSE; eight surfaces and six filters; no runtime CDN/Node | §13–§14; D2–D3 |
| 8 / VES-16 | Immutable route/config fingerprint; separate requested/resolved/served evidence; mismatch/unknown blocks affected capability | §8, §14–§16; R5 |
| 9 / VES-17 | Native audit; optional metadata-only Rust OTLP to Owner loopback collector; no backend/default worker exporter | §12, §15, §18 item 9; D19 |
| 10 / VES-18 | Resource budgets/measurement plan above; bounded projections; export-before-deletion eligibility, no unattended Tier-C cleanup | §7, §12, §14; D25 |
| 11 / VES-19 | Deterministic thirteen-point certificate keyed to material inputs; unknown remains unready; recertify only changed checks | §6; D27 |
| 12 / VES-20 | Heading plus fenced JSON marker, per-issue sequence/key, canonical semantic read-back; foreign text never reconstructs runtime | §10–§12; D21, D28 |
| 13 / VES-21 | Maintain own committed AGENTS as nonauthoritative data; no unsolicited generation in target repos; config and instruction discovery separately tested | §15, §18 item 13, §21; D21, D31, D35 |
| 14 / VES-22 | Stable goal/criterion IDs, hash text+checks+constraints, explicit direct/enabling edges, SQL drift predicates, persistent amendment counter | §6, §9, §12; D31, D33 |

## 10. Delivery dependencies and unsatisfied gates

The existing milestones remain the planning structure, not authorization to implement. Suggested design-to-proof order: state/audit/anchor foundation; atomic supervisor and crash harness; adapter/policy/pinning conformance; deterministic Chief/Linear/readiness; offline UI; independent integrated milestone-1 proof. Gate 3 must approve the sized plan before execution. VES-9 feeds VES-17/VES-21 and VES-25–VES-27; VES-11/VES-22 feed readiness and all lifecycle proofs. Do not create a parallel backlog. [§6, §19; D27]

**Blocking qualification gaps:** final-session tool-list canary; exact privileged-effect mediation/revocation; provider-served identity; secret-safe foreign runtime/egress behavior; atomic Job spawn/hard-crash evidence including broker limitations; end-to-end remote-write reconciliation. Existing VES-23–VES-31 own the relevant proof work. A credible design is not a pass for those issues.

Not designed to release precision yet: installer/signing/release pipeline, migration upgrade implementations, provider-specific attestation and OS egress sandbox, complete component API code, notification platform adapter, exact library lockfiles, or an upstream-compatible custom effect-tool implementation. These cannot be left hidden inside a supposedly ready implementation issue. The LLD defines contracts and rejection conditions; unresolved execution capabilities stay unavailable.

Gate 2 handoff requires independent review and Owner approval of both documents with exact source/digests and these limitations. Commit, PR, protected-main review/merge and branch protection remain unsatisfied; no issue is marked Done by this draft. [VES-8; §6, §21; D32, D35]

## 11. Research sources

Research date: 7 November 2026. Pinned source inspection and current docs establish contracts/intent only. Historical live results are in the four unchanged [evidence records](../evidence/).

- W1: [Microsoft Job-list and handle-list attributes](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-updateprocthreadattribute).
- W2: [Microsoft creation-time Job assignment and suspended-process gap](https://devblogs.microsoft.com/oldnewthing/20230209-00/?p=107812/).
- W3: [Job Objects, kill-on-close, descendant exceptions](https://learn.microsoft.com/en-us/windows/win32/procthread/job-objects); [CreateProcessW](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createprocessw).
- W4: [Job process-ID list](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-jobobject_basic_process_id_list); [Job accounting](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-jobobject_basic_accounting_information).
- W5: [TerminateJobObject](https://learn.microsoft.com/en-us/windows/win32/api/jobapi2/nf-jobapi2-terminatejobobject); [GetProcessTimes](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getprocesstimes).
- W6: [POSIX setsid](https://pubs.opengroup.org/onlinepubs/9799919799/functions/setsid.html).
- O1: [OpenCode pinned debug config](https://github.com/anomalyco/opencode/blob/v1.19.3/packages/opencode/src/cli/cmd/debug/config.ts#L6-L13); [debug agent inspection and execution](https://github.com/anomalyco/opencode/blob/v1.19.3/packages/opencode/src/cli/cmd/debug/agent.handler.ts#L40-L97).
- O2: [Final session tool preparation](https://github.com/anomalyco/opencode/blob/v1.19.3/packages/opencode/src/session/tools.ts#L390-L409); [request filtering](https://github.com/anomalyco/opencode/blob/v1.19.3/packages/opencode/src/session/llm/request.ts#L210-L215).
- O3: [Permission request/reply schema](https://github.com/anomalyco/opencode/blob/v1.19.3/packages/schema/src/v1/permission.ts#L19-L44); [reply once implementation](https://github.com/anomalyco/opencode/blob/v1.19.3/packages/opencode/src/permission/index.ts#L109-L145); [permission HTTP contract](https://github.com/anomalyco/opencode/blob/v1.19.3/packages/opencode/src/server/routes/instance/httpapi/groups/permission.ts#L11-L36).
- O4: [Export messages](https://github.com/anomalyco/opencode/blob/v1.19.3/packages/opencode/src/cli/cmd/export.ts#L283-L290); [assistant metadata](https://github.com/anomalyco/opencode/blob/v1.19.3/packages/schema/src/v1/session.ts#L453-L485); [selection before inference](https://github.com/anomalyco/opencode/blob/v1.19.3/packages/opencode/src/session/prompt.ts#L1186-L1201).
- O5: [Headless plugin pure guard](https://github.com/anomalyco/opencode/blob/v1.19.3/packages/opencode/src/plugin/index.ts#L170-L189); [runtime flag wiring](https://github.com/anomalyco/opencode/blob/v1.19.3/packages/opencode/src/effect/runtime-flags.ts#L16-L30); [XDG roots](https://github.com/anomalyco/opencode/blob/v1.19.3/packages/core/src/global.ts#L3-L28); [auth-content/file behavior](https://github.com/anomalyco/opencode/blob/v1.19.3/packages/opencode/src/auth/index.ts#L58-L66).
- L1: [Axum 0.8.4 documentation source](https://github.com/tokio-rs/axum/tree/axum-v0.8.4/axum/src); Context7 `/tokio-rs/axum/axum_v0_8_4`.
- L2: [Rusqlite 0.41.1 bundled SQLite](https://github.com/rusqlite/rusqlite/blob/v0.41.1/README.md); Context7 `/rusqlite/rusqlite/v0.41.1`.
- L3: [OTel Rust 0.33 release/stability](https://github.com/open-telemetry/opentelemetry-rust/blob/main/docs/release_0.33.md); [OTLP example](https://github.com/open-telemetry/opentelemetry-rust/blob/main/opentelemetry-otlp/examples/basic-otlp-http/src/main.rs); Context7 `/open-telemetry/opentelemetry-rust`, unpinned current documentation, exact release compatibility pending.
- L4: [Tokio Windows named-pipe ownership/I/O](https://docs.rs/tokio/latest/tokio/net/windows/named_pipe/struct.NamedPipeServer.html); [Rust OwnedHandle](https://doc.rust-lang.org/std/os/windows/io/struct.OwnedHandle.html).
