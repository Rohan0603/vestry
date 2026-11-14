//! Windows process-supervision boundary: Job objects, creation-time
//! assignment and process identity (LLD §8.4; T41 §4.4).
//!
//! This is the only module permitted to use `unsafe`. S0 ships no
//! implementation; the S0 Win32 Job smoke probe lives in the `vestry-testkit`
//! binary rather than here.
