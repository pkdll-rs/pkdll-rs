use std::mem;
use crate::utils::{cstring, rsa};

use crate::unwrap_or_err;

#[no_mangle]
pub extern "stdcall" fn rsa_pem_from_modulus(n_ptr: *const u16, e_ptr: *const u16) -> *const u16 {
    let n = cstring::from_ptr(n_ptr).unwrap();
    let n = unwrap_or_err!(base64::decode(n));

    let e = cstring::from_ptr(e_ptr).unwrap();
    let e = unwrap_or_err!(base64::decode(e));

    let pem_encoded = unwrap_or_err!(rsa::modulus_to_pem(n, e));

    let wstring = cstring::to_widechar(pem_encoded.trim_end());
    return mem::ManuallyDrop::new(wstring).as_ptr();
}

/// hash_type needed if you want to use oaep mode
#[no_mangle]
pub extern "stdcall" fn rsa_encrypt(data_ptr: *const u16, key_ptr: *const u16, hash_type_ptr: *const u16) -> *const u16 {
    let data = cstring::from_ptr(data_ptr).unwrap();
    let data = unwrap_or_err!(base64::decode(data));
    
    let key = cstring::from_ptr(key_ptr).unwrap();

    let hash_type = cstring::from_ptr(hash_type_ptr).unwrap();

    let encrypted = unwrap_or_err!(rsa::rsa_encrypt(data, key, hash_type));

    let wstring = cstring::to_widechar(&encrypted);
    return mem::ManuallyDrop::new(wstring).as_ptr();
}