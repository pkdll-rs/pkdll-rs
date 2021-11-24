use std::mem;
use base64;

use crate::utils::{cstring, aes};
use crate::unwrap_or_err;

/// inputs, outputs in base64
#[no_mangle]
pub extern "stdcall" fn aes_encrypt(data_ptr: *const u16, key_ptr: *const u16, iv_ptr: *const u16, mode_ptr: *const u16, padding_ptr: *const u16) -> *const u16 {
    let data = cstring::from_ptr(data_ptr).unwrap();
    let key = cstring::from_ptr(key_ptr).unwrap();
    let iv = cstring::from_ptr(iv_ptr).unwrap();
    let mode = cstring::from_ptr(mode_ptr).unwrap();
    let padding = cstring::from_ptr(padding_ptr).unwrap();

    let data = unwrap_or_err!(base64::decode(data));
    let key = unwrap_or_err!(base64::decode(key));
    let iv = unwrap_or_err!(base64::decode(iv));
    let encrypted = unwrap_or_err!(aes::aes_encrypt(data, key, iv, mode, padding));

    cstring::to_widechar(&encrypted).as_ptr()
}

/// inputs, outputs in base64
#[no_mangle]
pub extern "stdcall" fn aes_decrypt(data_ptr: *const u16, key_ptr: *const u16, iv_ptr: *const u16, mode_ptr: *const u16, padding_ptr: *const u16) -> *const u16 {
    let data = cstring::from_ptr(data_ptr).unwrap();
    let key = cstring::from_ptr(key_ptr).unwrap();
    let iv = cstring::from_ptr(iv_ptr).unwrap();
    let mode = cstring::from_ptr(mode_ptr).unwrap();
    let padding = cstring::from_ptr(padding_ptr).unwrap();

    let data = unwrap_or_err!(base64::decode(data));
    let key = unwrap_or_err!(base64::decode(key));
    let iv = unwrap_or_err!(base64::decode(iv));
    let decrypted = unwrap_or_err!(aes::aes_decrypt(data, key, iv, mode, padding));

    cstring::to_widechar(String::from_utf8_lossy(&decrypted).as_ref()).as_ptr()
}
