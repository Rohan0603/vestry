# OpenCode 1.19.3 live probe — plan (D20 phase 2b)

**Date:** 6 November 2026
**Status:** Authorised by the Owner in full ("I authorize you for everything") on 6 November 2026. Provider: **OpenRouter**, model: **`openrouter/openrouter/free`** (OpenRouter's free-model router), chosen by the Owner.
**Authoritative baseline:** `C:/Users/ponna/project/vestry/REQUIREMENTS.md` revision 0.9
**Result file:** `evidence/2026-11-06-opencode-1.19.3-live-probe.md`

This is the last outstanding item blocking the freeze to Baseline 1.0 (D23).

---

## Pre-flight findings (already established, read-only, no tokens spent)

| Check | Command | Result |
| -- | -- | -- |
| Version pinned matches installed | `opencode --version` | `1.19.3` — matches the tag the source was read at |
| Provider already authenticated | `opencode auth list` | `auth.json` holds **OpenAI oauth**, **OpenCode Go api**, **OpenRouter api**; environment holds **Google `GEMINI_API_KEY`**. No credential needed from the Owner |
| No policy env leaked into the shell | env name/set check | `OPENROUTER_API_KEY`, `OPENCODE_PERMISSION`, `OPENCODE_DB`, `OPENCODE_CONFIG_DIR`, `OPENCODE_DISABLE_PROJECT_CONFIG`, `OPENCODE_DISABLE_AUTOUPDATE` all **unset**. Clean baseline |
| Free-model router exists | `opencode models` | **`openrouter/openrouter/free`** present. Also `openrouter/openrouter/auto`, `/fusion`, `/bodybuilder`, `/pareto-code`. 364 OpenRouter models total, of which **11 are `:free`-suffixed** |
| `debug v2` cannot answer capability questions | `opencode debug v2` | Only 8,133 bytes: it dumps the **V2 core's own** two-provider catalog (`google`, `opencode`), not the full models.dev catalog. Useless for checking OpenRouter tool-calling support |

**Consequence:** whether `openrouter/openrouter/free` can perform tool calls at all is **unknown and becomes P0**. Every test that depends on the model invoking `bash` or `write` is gated on it. This is recorded as a probe risk, not hidden.

---

## Containment

Everything below runs inside a scratch directory. No real repository is touched, nothing is pushed, no network egress beyond the provider API and `127.0.0.1`.

```
PROBE = C:/Users/ponna/AppData/Local/Temp/vestry-probe-2026-11-06
  repo/     throwaway git repo, the working directory for every dispatch
  db/       OPENCODE_DB target — a scratch session store
  out/      captured stdout/stderr per test
```

Environment set on **every** dispatch, per §15:

| Variable | Value | Why |
| -- | -- | -- |
| `OPENCODE_DB` | `$PROBE/db/opencode.db` | Keeps the probe out of the Owner's real 2.25 GB store |
| `OPENCODE_DISABLE_AUTOUPDATE` | `1` | R5 — no version drift mid-probe |
| `OPENCODE_DISABLE_PROJECT_CONFIG` | `1` | §15(c) — the scratch repo's own config must not widen the envelope |

**Never passed:** `--auto`, `--yolo`, `--dangerously-skip-permissions`, `--cors`, `--mdns`, `--hostname` other than `127.0.0.1`.

**Fallback if the relocated DB loses auth:** re-run P0 without `OPENCODE_DB`. Whether `OPENCODE_DB` relocates the credential store or only the session store is itself a finding worth recording, because Vestry needs per-worker throwaway stores that still authenticate.

---

## Tests

### P0 — Provider and tool-calling smoke (gates everything)

1. `opencode run --model openrouter/openrouter/free "Reply with exactly one word: PONG"` — confirms auth survives the relocated DB, the free router is reachable, and the plain-text path works.
2. Same with `--format json` — captures the actual emitted event types against the six the source read predicted (`tool_use`, `step_start`, `step_finish`, `text`, `reasoning`, `error`).
3. `opencode run --model openrouter/openrouter/free "Create a file named pong.txt containing exactly PONG"` — **the gating question**: can the free router invoke a tool? Verified by checking `pong.txt` exists on disk, not by trusting the model's claim.

If 3 fails, P1/P2/P3/P7 cannot run through the model. Fallback: pin one of the 11 `:free` models individually and retry. If no free model can call tools, the probe escalates to the Owner for a paid model — **and that escalation is itself the finding**, because Vestry's Worker role requires reliable tool calling and "free tier" may not be able to supply it.

### P1 — Is `OPENCODE_PERMISSION` actually honoured by `run`?

Set `OPENCODE_PERMISSION='{"bash":{"*":"deny"}}'` and dispatch a prompt requiring bash. Expected: the tool is denied. Then `{"bash":{"*":"allow"}}` and the same prompt. Expected: it runs.

Also tests the §7 caveat: set `OPENCODE_PERMISSION='{invalid json'` and confirm the behaviour is *silent skip* (i.e. default policy applies) rather than a loud failure — which is why Vestry must verify the policy took effect rather than assume it.

**This is the single most important result in the probe.** The entire Tier model's enforceability through OpenCode rests on it.

### P2 — Descendant termination, empirically (the §11 question v1 never answered)

1. Snapshot the process list.
2. Dispatch a prompt that starts a long-lived child with its own descendants — e.g. `cmd /c ping -n 600 127.0.0.1` and, separately, a two-level tree so grandchild reaping is tested, not just child reaping.
3. While the child is live, abort the session (`POST /session/:id/abort` via a `serve` instance, since `run` has no abort surface).
4. Snapshot the process list again and diff.

**Pass condition:** the worker and *every* descendant is gone, verified by enumeration — not by the abort call returning 200. This is the invariant "an abort ACK is not descendant termination."

Also records the two known gaps from the source read: whether an already-exited Windows parent leaves orphans, and what happens on a hard kill (which runs no finalizer at all).

### P3 — Mid-run kill and restart reconciliation

Kill the OpenCode process outright mid-run (`taskkill /F`, no cooperation). Then attempt `--continue` and `--session <id>` resume against the scratch DB. Record whether state reconciles, duplicates, or silently loses the in-flight turn. This is milestone item 5 and the other thing v1 never proved.

### P4 — `export --sanitize` redaction adequacy

Run `opencode export <sessionID>` and `opencode export <sessionID> --sanitize` against a session whose transcript deliberately contains a fake secret-shaped token. Diff them. **The fake token is a marker string, never a real credential.** Determines whether export can be used for evidence capture under §15 or whether Vestry must redact itself.

### P5 — `stats` granularity

`opencode stats`, and with `--days`, `--models`, `--project`, `--tools`. Determines whether spend warnings (D16) can be built on `stats` or must come from the server event stream — the open question left by the source read, which found usage absent from `run --format json`.

### P6 — Server endpoints against a live instance

Start `opencode serve --hostname 127.0.0.1 --port 0`, discover the bound port from child stdout, then probe read-only: `/global/health`, `/global/event`, `/event`, `GET /global/config`. **Write endpoints are enumerated but not called** — specifically not `PATCH /global/config`, `/global/dispose`, `POST /global/upgrade`, or `PUT`/`DELETE /auth/:providerID`, because those mutate the Owner's real OpenCode installation. Also confirms whether basic auth is off by default with no `OPENCODE_SERVER_PASSWORD` set, and that `--mdns` rebinds away from loopback (tested by reading the bound address, not by exposing it).

### P7 — Which stack actually fires?

Under a real dispatch, determine whether the **V1 `ShellTool`** or the **V2 `BashTool`** handles a bash command. The source read established which module is *wired*; only execution establishes which one *runs*. Detected from the tool name in the event stream and the emitted arguments.

---

## What gets recorded

For each test: exact command, exit code, wall-clock duration, the model that actually served the request (the free router may rotate), raw output path under `$PROBE/out/`, and a pass/fail/indeterminate verdict with the reason.

No credential, key, token value or secret ever enters the evidence file. `$PROBE/out/` may contain transcript text; anything resembling a secret is a deliberately planted marker.

## What this probe cannot establish

- Behaviour under concurrency (three workers at once) — that is milestone 1, not the probe.
- Long-run stability or memory growth — needs hours, not minutes.
- Anything about a paid model's behaviour — the Owner chose the free router, so results are **specific to free-tier routing** and must be labelled as such. Free-tier models rotate and rate-limit, so a pass today is weaker evidence than a pass on a pinned model.
