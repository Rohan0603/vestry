//! Vestry daemon entrypoint — Phase B S0 stub.
//!
//! No product behaviour exists yet. The daemon fails closed with exit code 2 so
//! nothing that expects a working control plane can mistake this scaffold for
//! one (D48; T41 §6.0).

fn main() {
    eprintln!("vestryd: not implemented (Phase B S0)");
    std::process::exit(2);
}
