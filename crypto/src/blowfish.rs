use base64;
use winapi::um::winnt::PWSTR;

use crate::utils::{cstring, blowfish};
use crate::unwrap_or_err;

/// inputs, outputs in base64
#[no_mangle]
pub extern "stdcall" fn blowfish_encrypt(data_ptr: PWSTR, key_ptr: PWSTR, iv_ptr: PWSTR, mode_ptr: PWSTR, padding_ptr: PWSTR) -> PWSTR {
    let data = cstring::from_widechar_ptr(data_ptr);
    let key = cstring::from_widechar_ptr(key_ptr);
    let iv = cstring::from_widechar_ptr(iv_ptr);
    let mode = cstring::from_widechar_ptr(mode_ptr);
    let padding = cstring::from_widechar_ptr(padding_ptr);

    let data = unwrap_or_err!(base64::decode(data));
    let key = unwrap_or_err!(base64::decode(key));
    let iv = unwrap_or_err!(base64::decode(iv));
    let encrypted = unwrap_or_err!(blowfish::blowfish_encrypt(data, key, iv, mode, padding));

    cstring::to_widechar_ptr(&base64::encode(encrypted))
}

/// inputs, outputs in base64
#[no_mangle]
pub extern "stdcall" fn blowfish_decrypt(data_ptr: PWSTR, key_ptr: PWSTR, iv_ptr: PWSTR, mode_ptr: PWSTR, padding_ptr: PWSTR) -> PWSTR {
    let data = cstring::from_widechar_ptr(data_ptr);
    let key = cstring::from_widechar_ptr(key_ptr);
    let iv = cstring::from_widechar_ptr(iv_ptr);
    let mode = cstring::from_widechar_ptr(mode_ptr);
    let padding = cstring::from_widechar_ptr(padding_ptr);

    let data = unwrap_or_err!(base64::decode(data));
    let key = unwrap_or_err!(base64::decode(key));
    let iv = unwrap_or_err!(base64::decode(iv));
    let decrypted = unwrap_or_err!(blowfish::blowfish_decrypt(data, key, iv, mode, padding));

    cstring::to_widechar_ptr(&base64::encode(decrypted))
}
