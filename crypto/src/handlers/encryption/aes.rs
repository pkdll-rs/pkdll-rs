use crate::{
    imp::encryption,
    utils::base64,
    wstring::{FromWidechar, ToWidechar, LPCWSTR},
};

/// inputs, outputs in base64
#[no_mangle]
pub unsafe extern "stdcall" fn aes_encrypt(
    data_ptr: LPCWSTR,
    key_ptr: LPCWSTR,
    iv_ptr: LPCWSTR,
    mode_ptr: LPCWSTR,
    padding_ptr: LPCWSTR,
) -> LPCWSTR {
    let data = String::from_widechar_ptr(data_ptr);
    let key = String::from_widechar_ptr(key_ptr);
    let iv = String::from_widechar_ptr(iv_ptr);
    let mode = String::from_widechar_ptr(mode_ptr);
    let padding = String::from_widechar_ptr(padding_ptr);

    let mut data = base64::decode(data)?;
    let key = base64::decode(key)?;
    let iv = base64::decode(iv)?;
    let encrypted = encryption::aes_encrypt(data.as_mut_slice(), &key, &iv, &mode, &padding)?;

    base64::encode(encrypted).as_widechar_ptr()
}

/// inputs, outputs in base64
#[no_mangle]
pub unsafe extern "stdcall" fn aes_decrypt(
    data_ptr: LPCWSTR,
    key_ptr: LPCWSTR,
    iv_ptr: LPCWSTR,
    mode_ptr: LPCWSTR,
    padding_ptr: LPCWSTR,
) -> LPCWSTR {
    let data = String::from_widechar_ptr(data_ptr);
    let key = String::from_widechar_ptr(key_ptr);
    let iv = String::from_widechar_ptr(iv_ptr);
    let mode = String::from_widechar_ptr(mode_ptr);
    let padding = String::from_widechar_ptr(padding_ptr);

    let mut data = base64::decode(data)?;
    let key = base64::decode(key)?;
    let iv = base64::decode(iv)?;
    let decrypted = encryption::aes_decrypt(data.as_mut_slice(), &key, &iv, &mode, &padding)?;

    base64::encode(decrypted).as_widechar_ptr()
}
