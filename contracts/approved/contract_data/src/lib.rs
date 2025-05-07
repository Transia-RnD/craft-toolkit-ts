#![cfg_attr(target_arch = "wasm32", no_std)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

use xrpl_std::host::trace::{trace, trace_num};
use xrpl_std::core::types::account_id::AccountID;
use xrpl_std::core::data::codec::{STDataManager};
use xrpl_std::core::error_codes::{INTERNAL_ERROR};

#[unsafe(no_mangle)]
pub extern "C" fn create() -> i32 {

    const ACCOUNT: [u8; 20] = [
        0xAE, 0x12, 0x3A, 0x85, 0x56, 0xF3, 0xCF, 0x91, 0x15, 0x47, 
        0x11, 0x37, 0x6A, 0xFB, 0x0F, 0x89, 0x4F, 0x83, 0x2B, 0x3D
    ];
    let account = AccountID(ACCOUNT);

    let mut manager = STDataManager::with_account(account);

    let _ = manager.set_uint32("count", 3);
    let _ = manager.set_uint32("total", 12);

    // Add account ID
    const DESTINATION: [u8; 20] = [
        0x05, 0x96, 0x91, 0x5C, 0xFD, 0xEE, 0xE3, 0xA6, 0x95, 0xB3,
        0xEF, 0xD6, 0xBD, 0xA9, 0xAC, 0x78, 0x8A, 0x36, 0x8B, 0x7B
    ];
    
    let _ = manager.set_account("destination", DESTINATION);

    match manager.save_data() {
        Ok(()) => {},
        Err(e) => {
            return e;
        }
    }

    if let Some(count_val) = manager.get_uint32("count") {
        let _ = trace_num("Read back count: {}", count_val.into());
    } else {
        let _ = trace("Failed to read back count");
        return INTERNAL_ERROR;
    }
    return 0;
}

#[unsafe(no_mangle)]
pub extern "C" fn update() -> i32 {
    const ACCOUNT: [u8; 20] = [
        0xAE, 0x12, 0x3A, 0x85, 0x56, 0xF3, 0xCF, 0x91, 0x15, 0x47, 
        0x11, 0x37, 0x6A, 0xFB, 0x0F, 0x89, 0x4F, 0x83, 0x2B, 0x3D
    ];
    let account = AccountID(ACCOUNT);
    let mut manager = STDataManager::with_account(account);
    match manager.get_data() {
        Ok(()) => {},
        Err(e) => {
            return e;
        }
    }
    
    let _ = manager.set_uint32("count", 4);

    match manager.save_data() {
        Ok(()) => {},
        Err(e) => {
            return e;
        }
    }
    
    if let Some(count_val) = manager.get_uint32("count") {
        let _ = trace_num("Read back count: {}", count_val.into());
    } else {
        let _ = trace("Failed to read back count");
        return INTERNAL_ERROR;
    }
    return 0;
}