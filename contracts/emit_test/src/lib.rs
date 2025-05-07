#![cfg_attr(target_arch = "wasm32", no_std)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

use xrpl_wasm_std::core::current_tx::contract_call::{ContractCall, get_current_contract_call};
use xrpl_wasm_std::core::current_tx::traits::TransactionCommonFields;
use xrpl_wasm_std::core::submit::inner_objects::build_memo;
use xrpl_wasm_std::core::types::transaction_type::TransactionType;
use xrpl_wasm_std::core::types::account_id::AccountID;
use xrpl_wasm_std::host::{add_txn_field, build_txn, emit_built_txn};
use xrpl_wasm_std::host::trace::{trace, trace_num, trace_data, DataRepr};
use xrpl_wasm_std::sfield;
use xrpl_wasm_std::sflags;
use xrpl_wasm_std::core::ledger_objects::nft::get_nft;
use xrpl_wasm_std::core::type_codes::{
    STI_UINT256
};
use xrpl_wasm_std::host::{function_param};
use xrpl_wasm_std::host::{Error, Result, Result::Err, Result::Ok};

// ============================================================================
// Constants
// ============================================================================

/// Buffer sizes
mod buffer_sizes {
    pub const DESTINATION: usize = 21;
    pub const NFT_ID: usize = 32;
}

/// Custom error code for transaction failures
const CUSTOM_ERROR_CODE: i32 = -18;

unsafe fn add_flags_field(txn_index: i32, flags: u32) -> i32 {
    // Always include tfInnerBatchTxn flag
    let combined_flags = flags | sflags::tfInnerBatchTxn;

    // Convert u32 to bytes (big-endian for XRP Ledger)
    let flags_bytes = combined_flags.to_be_bytes();
    
    add_txn_field(
        txn_index,
        sfield::Flags,
        flags_bytes.as_ptr(),
        flags_bytes.len()
    )
}

unsafe fn add_amount_field(txn_index: i32) -> i32 {
    // 192 drops encoded as XRPL Amount
    const AMOUNT_BYTES: [u8; 8] = [
        0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xC0
    ];
    
    add_txn_field(
        txn_index, 
        sfield::Amount, 
        AMOUNT_BYTES.as_ptr(), 
        AMOUNT_BYTES.len()
    )
}

unsafe fn add_destination_field(txn_index: i32, destination: &AccountID) -> i32 {
    let mut dest_buffer = [0u8; buffer_sizes::DESTINATION];
    dest_buffer[0] = 0x14; // Length prefix for 20-byte account
    dest_buffer[1..21].copy_from_slice(&destination.0);
    
    add_txn_field(
        txn_index,
        sfield::Destination,
        dest_buffer.as_ptr(),
        dest_buffer.len()
    )
}

unsafe fn add_nftid_field(txn_index: i32, nft_id: &[u8; 32]) -> i32 {
    add_txn_field(
        txn_index,
        sfield::NFTokenID,
        nft_id.as_ptr(),
        nft_id.len()
    )
}

#[unsafe(no_mangle)]
pub extern "C" fn emit() -> i32 {
    // Get contract context
    let contract_call: ContractCall = get_current_contract_call();
    let account = contract_call.get_account().unwrap();

    let mut nft_id = [0x00; 32];
    let output_len = unsafe { function_param(2, STI_UINT256.into(), nft_id.as_mut_ptr(), nft_id.len()) };
    let _ = trace_num("UINT256 Value Len:", output_len as i64);
    // as hex
    let _ = trace_data("UINT256 Hex:", &nft_id[0..32], DataRepr::AsHex);

    let _ = unsafe { get_nft(&account, &nft_id) };

    // Initialize NFTCreateOffer transaction
    let txn_index = 0;
    let build_result = unsafe { build_txn(TransactionType::NFTokenCreateOffer as i32) };
    if build_result < 0 {
        let _ = trace_num("build_result: {}", build_result.into());
        return -1;
    }

    // Build transaction fields
    unsafe {
        // Add amount field
        if add_amount_field(txn_index) < 0 {
            return -2;
        }

        // Add destination field
        if add_destination_field(txn_index, &account) < 0 {
            return -3;
        }

        // Add flags field with tfSell flag
        if add_flags_field(txn_index, sflags::tfSellNFToken) < 0 {
            return -4;
        }

        // Add nftid field
        if add_nftid_field(txn_index, &nft_id) < 0 {
            return -5;
        }

        // Emit the completed transaction
        let emission_result = emit_built_txn(txn_index);
        if emission_result < 0 {
            let _ = trace_num("emission_result: {}", emission_result.into());
            return emission_result;
        }
    }

    0 // Success
}