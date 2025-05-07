#![allow(unused_imports)]
#![cfg_attr(target_arch = "wasm32", no_std)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

use xrpl_wasm_std::core::params::function::{get_function_param, safe_get_function_param};
use xrpl_wasm_std::core::types::account_id::AccountID;
use xrpl_wasm_std::core::types::amount::token_amount::TokenAmount;
use xrpl_wasm_std::host::trace::{trace_num, DataRepr};

const SUCCESS: i32 = 0;
const BAD_PARAM: i32 = -1;

#[unsafe(no_mangle)]
pub extern "C" fn easymode() -> i32 {

    // Get: AccountID
    let account = safe_get_function_param::<AccountID>(0);

    // Get: TokenAmount
    let amount = match get_function_param::<TokenAmount>(1) {
        Ok(a) => a,
        Err(err) => {
            let _ = trace_num("`TokenAmount` Parameter Error Code:", err as i64);
            return BAD_PARAM;
        }
    };

    // Transfer: from the "contract" to the "account"
    let tx_id = amount.transfer(&account);
    if tx_id < 0 {
        let _ = trace_num("Transfer Error Code:", tx_id as i64);
        return tx_id;
    }

    return SUCCESS;
}
