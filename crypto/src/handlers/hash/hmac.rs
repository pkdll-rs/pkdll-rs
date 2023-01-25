use crate::{
    imp::hashing,
    utils::base64,
    wstring::{FromWidechar, ToWidechar, LPCWSTR},
};

#[no_mangle]
pub unsafe extern "stdcall" fn hmac(
    hash_type: LPCWSTR,
    data_ptr: LPCWSTR,
    key_ptr: LPCWSTR,
) -> LPCWSTR {
    let data = String::from_widechar_ptr(data_ptr);
    let data = base64::decode(data)?;

    let key = String::from_widechar_ptr(key_ptr);
    let key = base64::decode(key)?;

    let hash_type = String::from_widechar_ptr(hash_type);

    let hashed = hashing::make_hmac(data, key, &hash_type)?;

    base64::encode(hashed).as_widechar_ptr()
}
