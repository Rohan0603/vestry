//! Vestry core library.
//!
//! Phase B S0 scaffold only: this crate declares the module layout agreed for
//! Phase B so the two implementation tracks can own disjoint subtrees (T41
//! §6.3). **No product behaviour and no contract types live here yet** — the
//! C1–C8 contract freeze is S1 (`contracts`). Every module below is a stub with
//! a doc comment naming its design source.
//!
//! Unsafe policy: the crate deliberately does **not** carry
//! `#![forbid(unsafe_code)]`. Win32 creation-time Job assignment and process
//! control require `unsafe`, which is confined to `platform::windows`
//! (T41 §4.4). `unsafe_op_in_unsafe_fn` is denied so any future unsafe block
//! must state its own obligation.

#![deny(unsafe_op_in_unsafe_fn)]

pub mod adapter;
pub mod audit;
pub mod canary;
pub mod contracts;
pub mod daemon;
pub mod fake_adapter;
pub mod ndjson;
pub mod pin;
pub mod platform;
pub mod policy;
pub mod recovery;
pub mod state;
pub mod supervisor;

/// Compile-time check that the S0 module layout is wired.
///
/// This asserts the scaffold builds and the test harness runs; it carries no
/// product behaviour.
#[cfg(test)]
mod tests {
    #[test]
    fn workspace_layout_compiles() {}
}
