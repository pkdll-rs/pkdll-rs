use crate::{
    imp::encryption,
    utils::base64,
    wstring::{FromWidechar, ToWidechar, LPCWSTR},
};

#[no_mangle]
pub unsafe extern "stdcall" fn rsa_pem_from_modulus(n_ptr: LPCWSTR, e_ptr: LPCWSTR) -> LPCWSTR {
    let n = String::from_widechar_ptr(n_ptr);
    let n = base64::decode(n)?;

    let e = String::from_widechar_ptr(e_ptr);
    let e = base64::decode(e)?;

    let pem_encoded = encryption::modulus_to_pem(&n, &e)?;
    pem_encoded.trim_end().as_widechar_ptr()
}

/// hash_type needed if you want to use oaep mode
#[no_mangle]
pub unsafe extern "stdcall" fn rsa_encrypt(
    data_ptr: LPCWSTR,
    key_ptr: LPCWSTR,
    hash_type_ptr: LPCWSTR,
) -> LPCWSTR {
    let data = String::from_widechar_ptr(data_ptr);
    let data = base64::decode(data)?;

    let key = String::from_widechar_ptr(key_ptr);
    let hash_type = String::from_widechar_ptr(hash_type_ptr);

    let encrypted = encryption::rsa_encrypt(&data, &key, &hash_type)?;
    base64::encode(encrypted).as_widechar_ptr()
}

/// hash_type needed if you want to use oaep mode
#[no_mangle]
pub unsafe extern "stdcall" fn rsa_decrypt(
    data_ptr: LPCWSTR,
    key_ptr: LPCWSTR,
    hash_type_ptr: LPCWSTR,
) -> LPCWSTR {
    let data = String::from_widechar_ptr(data_ptr);
    let data = base64::decode(data)?;

    let key = String::from_widechar_ptr(key_ptr);
    let hash_type = String::from_widechar_ptr(hash_type_ptr);

    let decrypted = encryption::rsa_decrypt(&data, &key, &hash_type)?;
    base64::encode(decrypted).as_widechar_ptr()
}

#[no_mangle]
pub unsafe extern "stdcall" fn rsa_sign(
    data_ptr: LPCWSTR,
    key_ptr: LPCWSTR,
    hash_type_ptr: LPCWSTR,
    mode_ptr: LPCWSTR,
) -> LPCWSTR {
    let data = String::from_widechar_ptr(data_ptr);
    let data = base64::decode(data)?;

    let key = String::from_widechar_ptr(key_ptr);

    let hash_type = String::from_widechar_ptr(hash_type_ptr);

    let mode = String::from_widechar_ptr(mode_ptr);

    let signed = encryption::rsa_sign(&data, &key, &hash_type, &mode)?;

    base64::encode(signed).as_widechar_ptr()
}
