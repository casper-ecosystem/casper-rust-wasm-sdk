// getter_with_clone expands to .clone() on Copy fields (clippy::clone_on_copy on 1.99+).
#![cfg_attr(feature = "js", allow(clippy::clone_on_copy))]
// wasm-bindgen generated code triggers drop_non_drop with getter_with_clone.
#![cfg_attr(
    all(feature = "js", target_arch = "wasm32"),
    allow(clippy::drop_non_drop)
)]

pub mod helpers;
pub mod types;

pub(crate) mod sdk;
pub use sdk::*;

#[cfg(feature = "js")]
pub(crate) mod js;
#[cfg(feature = "js")]
pub use js::externs as debug;

/// Native log/error when the `js` feature (and `js::externs`) is off.
#[cfg(not(feature = "js"))]
pub(crate) mod debug {
    pub fn log(s: &str) {
        println!("log wasm {s}");
    }

    pub fn error(s: &str) {
        println!("error wasm {s}");
    }
}
