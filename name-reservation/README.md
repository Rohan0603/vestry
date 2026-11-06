# Securing the Vestry name — prepared actions

Everything here is prepared for you to execute. I cannot register a domain, create a
GitHub organisation, or publish a crate without your accounts and your authorisation.

Verified 6 November 2026. Re-verify immediately before acting — `vestry` was free on
crates.io at the time of writing and may not stay that way.

## 1. Reserve the crate name (do this first)

`vestry` is free on crates.io with no adjacent matches. This is the one surface that is
both free and decisive for a Rust project.

A placeholder crate is prepared in `vestry-crate/`. To publish it:

```
cd vestry-crate
cargo login          # paste your crates.io token; it is stored locally, not in this repo
cargo publish
```

Then confirm by fetching `https://crates.io/api/v1/crates/vestry` and checking that
`owner` is you. Do not treat a successful `cargo publish` exit as proof until you have
read it back — the same discipline §10 of the requirements baseline applies to Linear.

Note the placeholder declares `repository` and `homepage` pointing at github.com/usevestry/vestry.
Create the organisation (step 2) before publishing, or edit those fields first.

## 2. Create the GitHub organisation

Available at time of checking: `usevestry`, `getvestry`, `vestry-rs`, `vestry-dev`,
`vestryapp`, `vestrydev`, `thevestry`. Taken: `vestry`, `vestryhq`.

Recommendation: **`vestry-rs`** if the project is presented as a Rust tool, otherwise
**`usevestry`**. Create it at https://github.com/account/organizations/new.

Availability was determined by HTML probe (`github.com/<handle>` returning 404 versus 200)
because the unauthenticated API rate limit was exhausted during the search. That method is
reliable for existence but cannot tell you whether a free handle is worth taking quickly.

## 3. Register a domain

Probably available per RDAP: `vestry.io`, `vestry.sh`, `vestry.tools`, `vestry.run`,
`vestry.build`.

Already registered: `vestry.com`, `.org`, `.ai`, `.dev`, `.app`, `vestryhq.com`,
`usevestry.com`, `getvestry.com`.

**Confirm at a registrar before paying.** RDAP "not found" can mean the registry lacks
RDAP support rather than that the domain is unregistered. `.io` and `.sh` are the two
most likely to give a false negative here.

Recommendation: `vestry.io` if available, otherwise `vestry.sh` — the latter reads
naturally for a local daemon.

## 4. Do not rename again

This product has been renamed three times: Orchestrator → Agent Orchestrator →
AgentConductor (1 November 2026) → Vestry. The AgentConductor rename is why ORC-46
exists and why the v1 approval register needed reconciling. Renaming mid-flight costs
governance churn every time, and §20 of the baseline records this as a binding action.

## Known collisions, for the record

- `getvestry.com` is a live worship-planning app for worship leaders. Different domain
  entirely, but it owns the exact-name search result.
- npm has a `@vestry` scope and a `vestry-telemetry` package belonging to an unrelated
  marketplace product. Irrelevant unless Vestry ever publishes to npm, which it will not.
- No developer tool, AI agent framework, or governance product uses the name.
