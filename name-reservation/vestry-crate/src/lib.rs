//! Vestry — a durable local control plane for autonomous software engineering.
//!
//! This crate currently exists to reserve the `vestry` name on crates.io. It contains no
//! functional code. The requirements baseline for the project is maintained outside this
//! crate; see the repository for the authoritative document.
//!
//! Do not depend on this crate yet. Nothing here is stable, and nothing here works.

#![forbid(unsafe_code)]

/// Returns the reservation marker for this crate.
///
/// Present only so that the crate has a public API surface and a testable item. It carries
/// no meaning beyond proving the crate builds.
pub const NAME_RESERVATION: &str = "vestry";

#[cfg(test)]
mod tests {
    use super::NAME_RESERVATION;

    #[test]
    fn reservation_marker_is_the_crate_name() {
        assert_eq!(NAME_RESERVATION, "vestry");
    }
}
