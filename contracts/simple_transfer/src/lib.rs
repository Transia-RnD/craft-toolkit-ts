#![allow(unused_imports)]
#![cfg_attr(target_arch = "wasm32", no_std)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

use xrpl_wasm_stdlib::wasm_export;
use xrpl_wasm_stdlib::core::params::function::{get_function_param, safe_get_function_param};
use xrpl_wasm_stdlib::core::types::account_id::AccountID;
use xrpl_wasm_stdlib::core::types::amount::Amount;
use xrpl_wasm_stdlib::host::trace::{trace, trace_num, DataRepr};

const SUCCESS: i32 = 0;

fn exit(message: &str, error_code: i32) -> i32 {
    let _ = trace(message);
    let _ = trace_num("Error Code:", error_code as i64);
    error_code
}

#[wasm_export(
    exit = exit,
    instance(initialBalance: Amount)
)]
pub extern "C" fn simple_transfer(
    account: AccountID,
    amount: Amount
) -> i32 {
    // let _ = trace_num("AccountID to transfer to:", initialBalance.to_i64());

    let tx_id = amount.transfer(&account);
    if tx_id < 0 {
        let _ = trace_num("AMOUNT Transfer Error Code:", tx_id as i64);
        return exit("Transfer failed", tx_id);
    }

    return exit("Redirect successful", SUCCESS);
}
