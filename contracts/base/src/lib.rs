#![cfg_attr(target_arch = "wasm32", no_std)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

use xrpl_wasm_std::host::trace::{trace};

#[unsafe(no_mangle)]
pub extern "C" fn base() -> i32 {
    let _ = trace("$$$$$ STARTING WASM EXECUTION $$$$$");
    0
}