#![cfg_attr(target_arch = "wasm32", no_std)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

use xrpl_std::host::trace::{trace, trace_num};
use xrpl_std::core::types::account_id::AccountID;
use xrpl_std::core::data::codec::{STDataManager};
use xrpl_std::core::error_codes::{INTERNAL_ERROR};

#[unsafe(no_mangle)]
pub extern "C" fn create() -> i32 {

    const DATA: [u8; 28] = [
        0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01
        0xAE, 0x12, 0x3A, 0x85, 0x56, 0xF3, 0xCF, 0x91, 0x15, 0x47, 
        0x11, 0x37, 0x6A, 0xFB, 0x0F, 0x89, 0x4F, 0x83, 0x2B, 0x3D
    ];


    // typedef struct {
    //     uint32 count;
    //     uint32 total;
    //     uint8[20] destination
    // } payment_t

    // ContractData.count
    return 0;
}

#![cfg_attr(target_arch = "wasm32", no_std)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

use xrpl_std::host::trace::{trace_num, trace_hex};

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct ModelBuffer {
    pub updated_time: u64,
    pub updated_by: [u8; 32],
    pub message: [u8; 32],
}

#[unsafe(no_mangle)]
pub extern "C" fn create() -> i32 {
    const DATA: [u8; 72] = [
        // updated_time (8 bytes)
        0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80,
        // updated_by (32 bytes)
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10,
        0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18,
        0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F, 0x20,
        // message (32 bytes)
        0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x00, 0x11,
        0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99,
        0xA1, 0xB2, 0xC3, 0xD4, 0xE5, 0xF6, 0x07, 0x18,
        0x29, 0x3A, 0x4B, 0x5C, 0x6D, 0x7E, 0x8F, 0x90
    ];

    // Cast DATA directly onto ModelBuffer
    unsafe {
        let model = &*(DATA.as_ptr() as *const ModelBuffer);
        trace_num("Updated time", model.updated_time as i64);
        trace_hex("Updated by", &model.updated_by);
        trace_hex("Message", &model.message);
    }

    0
}