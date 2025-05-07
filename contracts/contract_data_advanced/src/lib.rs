#![cfg_attr(target_arch = "wasm32", no_std)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

use xrpl_std::host::trace::{trace, trace_num, trace_data, DataRepr};
use xrpl_std::host::{function_param};
use xrpl_std::core::types::account_id::AccountID;
use xrpl_std::core::data::codec_v2::{get_uint32, set_uint32, get_account, set_account, set_nested_uint8, get_nested_uint8};
use xrpl_std::core::error_codes::{INTERNAL_ERROR};
use xrpl_std::core::type_codes::{
    STI_UINT32, STI_ACCOUNT
};
use xrpl_std::core::current_tx::contract_call::{ContractCall, get_current_contract_call};
use xrpl_std::core::current_tx::traits::{ContractCallFields};

#[unsafe(no_mangle)]
pub extern "C" fn test() -> i32 {
    let mut buf = [0x00; 20];
    let output_len = unsafe { function_param(0, STI_ACCOUNT.into(), buf.as_mut_ptr(), buf.len()) };
    let other_contract_account = AccountID::from(buf);
    let _ = trace_data("FROM Account:", &other_contract_account.0, DataRepr::AsHex);

    let contract_call: ContractCall = get_current_contract_call();
    let current_contract_account = contract_call.get_contract_account().unwrap();
    let _ = trace_data("CONTRACT ACCOUNT:", &current_contract_account.0, DataRepr::AsHex);

    // UINT32
    let mut buf = [0x00; 4];
    let output_len = unsafe { function_param(1, STI_UINT32.into(), buf.as_mut_ptr(), buf.len()) };
    let value = u32::from_le_bytes(buf);
    if let Err(e) = set_uint32(&current_contract_account, "count", value) {
        return e;
    }

    // Read Other Contract Data
    if let Some(count_val) = get_uint32(&other_contract_account, "count") {
        let _ = trace_num("Read back count: {}", count_val.into());
    } else {
        let _ = trace("Failed to read back count");
        return INTERNAL_ERROR;
    }
    
    0
}