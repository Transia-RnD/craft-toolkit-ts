#![cfg_attr(target_arch = "wasm32", no_std)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

use xrpl_std::core::types::account_id::AccountID;
use xrpl_std::core::types::public_key::PublicKey;
use xrpl_std::core::types::amount::token_amount::TokenAmount;
use xrpl_std::host::trace::{DataRepr, trace, trace_data, trace_num};
use xrpl_std::host::{instance_param, emit_txn};
use xrpl_std::core::current_tx::contract_call::{ContractCall, get_current_contract_call};
use xrpl_std::core::current_tx::traits::{ContractCallFields, TransactionCommonFields};
use xrpl_std::core::ledger_objects::account::get_account_sequence;
use xrpl_std::core::submit::common::{CommonFields, SerializationBuffer};
use xrpl_std::core::submit::contract_call::{ContractCallTemplate, ContractCallTxn, ParameterValue, Parameter};
use xrpl_std::core::type_codes::{
    STI_ACCOUNT, STI_UINT32
};
use xrpl_std::host::{function_param};
use xrpl_std::core::data::codec_v2::{set_uint32};

const CUSTOM_ERROR_CODE: i32 = -18;

#[unsafe(no_mangle)]
pub extern "C" fn emit() -> i32 {
    let _ = trace("$$$$$ STARTING WASM EXECUTION $$$$$");
    let contract_call: ContractCall = get_current_contract_call();
    let account = contract_call.get_account().unwrap();
    let _ = trace_data("FROM ACCOUNT:", &account.0, DataRepr::AsHex);
    let contract_account = contract_call.get_contract_account().unwrap();
    let _ = trace_data("CONTRACT ACCOUNT:", &contract_account.0, DataRepr::AsHex);

    // Create signing public key (empty for contract calls)
    const PUBKEY_SECP256K1: &[u8] = &[0x00];
    let pubkey = PublicKey::from(PUBKEY_SECP256K1);
    let _ = trace_data("  SigningPubKey:", &pubkey.0, DataRepr::AsHex);

    // Create fee amount (0 XRP for now)
    const FEE: [u8; 8] = [
        0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    // let mut fee_bytes = [0u8; 48];
    // fee_bytes[0..8].copy_from_slice(&FEE);
    let fee = match TokenAmount::from_bytes(&FEE) {
        Ok(f) => f,
        Err(_) => {
            let _ = trace("Failed to parse fee TokenAmount");
            return CUSTOM_ERROR_CODE;
        }
    };

    // Get next sequence number
    let next_sequence = get_account_sequence(&contract_account);
    let sequence_value = next_sequence.unwrap_or(0);
    let _ = trace_num("Next Sequence:", sequence_value as i64);

    // Create common transaction fields
    let common = CommonFields {
        transaction_type: 77, // ContractCall
        sequence: sequence_value,
        fee: fee,
        account: contract_account,
        signing_pub_key: pubkey,
        flags: 536870912, // Standard flags
    };

    let function_name = b"emit";
    
    let mut buf = [0x00; 20];
    let output_len = unsafe { function_param(1, STI_ACCOUNT.into(), buf.as_mut_ptr(), buf.len()) };
    let other_contract_account = AccountID::from(buf);

    let mut buf = [0x00; 4];
    let output_len = unsafe { function_param(2, STI_UINT32.into(), buf.as_mut_ptr(), buf.len()) };
    let value = u32::from_le_bytes(buf);
    if let Err(e) = set_uint32(&contract_account, "count", value) {
        return e;
    }

    let parameters = [
        Parameter {
            parameter_flag: None,
            parameter_name: Some("account"),
            parameter_value: Some(ParameterValue::new_account(contract_account)),
            parameter_type: Some("ACCOUNT"),
        },
        Parameter {
            parameter_flag: None,
            parameter_name: Some("uint32"),
            parameter_value: Some(ParameterValue::new_uint32(12)),
            parameter_type: Some("UINT32"),
        }
    ];

    let contract_call = ContractCallTemplate {
        contract_account: other_contract_account,
        function_name,
        parameters: Some(&parameters),
        // parameters: None,
        computation_allowance: 1000000, // Example value
        // ... other fields
    };

    // Create contract call transaction
    let txn = ContractCallTxn::new(common, contract_call);

    // Serialize and submit transaction
    let mut txn_buf = SerializationBuffer::new();
    match txn.serialize(&mut txn_buf) {
        Ok(()) => {
            let ter_result = unsafe { 
                emit_txn(
                    txn_buf.as_slice().as_ptr(), 
                    txn_buf.len(),
                )
            };
            if (ter_result != 0) {
                let _ = trace("Transaction submission failed");
                return CUSTOM_ERROR_CODE;
            }

            // Trace the results
            let _ = trace_data("Serialized ContractCallTxn:", txn_buf.as_slice(), DataRepr::AsHex);
            let _ = trace_num("Submit Result:", ter_result.into());
        }
        Err(e) => {
            let _ = trace("Serialization error occurred");
            return CUSTOM_ERROR_CODE;
        }
    }

    0 // Return success
}