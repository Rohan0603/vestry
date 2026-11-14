//! Platform-specific process and OS primitives. Track 2 owns this subtree
//! (T41 §6.3).
//!
//! Unsafe policy: `unsafe` is confined to [`windows`] (T41 §4.4). No platform
//! behaviour exists at S0.

pub mod posix;
pub mod windows;
