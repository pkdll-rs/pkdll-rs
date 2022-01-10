use winapi::um::winnt::LPCWSTR;

use crate::unwrap_or_err;
use crate::utils::{aes, cstring};

/// inputs, outputs in base64
#[no_mangle]
pub extern "stdcall" fn aes_encrypt(
    data_ptr: LPCWSTR,
    key_ptr: LPCWSTR,
    iv_ptr: LPCWSTR,
    mode_ptr: LPCWSTR,
    padding_ptr: LPCWSTR,
) -> LPCWSTR {
    let data = cstring::from_widechar_ptr(data_ptr);
    let key = cstring::from_widechar_ptr(key_ptr);
    let iv = cstring::from_widechar_ptr(iv_ptr);
    let mode = cstring::from_widechar_ptr(mode_ptr);
    let padding = cstring::from_widechar_ptr(padding_ptr);

    let data = unwrap_or_err!(base64::decode(data));
    let key = unwrap_or_err!(base64::decode(key));
    let iv = unwrap_or_err!(base64::decode(iv));
    let encrypted = unwrap_or_err!(aes::aes_encrypt(data, key, iv, mode, padding));

    cstring::to_widechar_ptr(&base64::encode(encrypted))
}

/// inputs, outputs in base64
#[no_mangle]
pub extern "stdcall" fn aes_decrypt(
    data_ptr: LPCWSTR,
    key_ptr: LPCWSTR,
    iv_ptr: LPCWSTR,
    mode_ptr: LPCWSTR,
    padding_ptr: LPCWSTR,
) -> LPCWSTR {
    let data = cstring::from_widechar_ptr(data_ptr);
    let key = cstring::from_widechar_ptr(key_ptr);
    let iv = cstring::from_widechar_ptr(iv_ptr);
    let mode = cstring::from_widechar_ptr(mode_ptr);
    let padding = cstring::from_widechar_ptr(padding_ptr);

    let data = unwrap_or_err!(base64::decode(data));
    let key = unwrap_or_err!(base64::decode(key));
    let iv = unwrap_or_err!(base64::decode(iv));
    let decrypted = unwrap_or_err!(aes::aes_decrypt(data, key, iv, mode, padding));

    cstring::to_widechar_ptr(&base64::encode(decrypted))
}
