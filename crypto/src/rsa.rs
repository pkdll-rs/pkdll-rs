use winapi::um::winnt::PWSTR;

use crate::utils::{cstring, rsa};

use crate::unwrap_or_err;

#[no_mangle]
pub extern "stdcall" fn rsa_pem_from_modulus(n_ptr: PWSTR, e_ptr: PWSTR) -> PWSTR {
    let n = cstring::from_widechar_ptr(n_ptr);
    let n = unwrap_or_err!(base64::decode(n));

    let e = cstring::from_widechar_ptr(e_ptr);
    let e = unwrap_or_err!(base64::decode(e));

    let pem_encoded = unwrap_or_err!(rsa::modulus_to_pem(n, e));

    cstring::to_widechar_ptr(pem_encoded.trim_end())
}

/// hash_type needed if you want to use oaep mode
#[no_mangle]
pub extern "stdcall" fn rsa_encrypt(data_ptr: PWSTR, key_ptr: PWSTR, hash_type_ptr: PWSTR) -> PWSTR {
    let data = cstring::from_widechar_ptr(data_ptr);
    let data = unwrap_or_err!(base64::decode(data));
    
    let key = cstring::from_widechar_ptr(key_ptr);

    let hash_type = cstring::from_widechar_ptr(hash_type_ptr);

    let encrypted = unwrap_or_err!(rsa::rsa_encrypt(data, key, hash_type));

    cstring::to_widechar_ptr(&base64::encode(encrypted))
}

/// hash_type needed if you want to use oaep mode
#[no_mangle]
pub extern "stdcall" fn rsa_decrypt(data_ptr: PWSTR, key_ptr: PWSTR, hash_type_ptr: PWSTR) -> PWSTR {
    let data = cstring::from_widechar_ptr(data_ptr);
    let data = unwrap_or_err!(base64::decode(data));
    
    let key = cstring::from_widechar_ptr(key_ptr);

    let hash_type = cstring::from_widechar_ptr(hash_type_ptr);

    let decrypted = unwrap_or_err!(rsa::rsa_decrypt(data, key, hash_type));

    cstring::to_widechar_ptr(&base64::encode(decrypted))
}

#[no_mangle]
pub extern "stdcall" fn rsa_sign(data_ptr: PWSTR, key_ptr: PWSTR, hash_type_ptr: PWSTR, mode_ptr: PWSTR) -> PWSTR {
    let data = cstring::from_widechar_ptr(data_ptr);
    let data = unwrap_or_err!(base64::decode(data));
    
    let key = cstring::from_widechar_ptr(key_ptr);

    let hash_type = cstring::from_widechar_ptr(hash_type_ptr);

    let mode = cstring::from_widechar_ptr(mode_ptr);

    let signed = unwrap_or_err!(rsa::rsa_sign(data, key, hash_type, mode));

    cstring::to_widechar_ptr(&base64::encode(signed))
}