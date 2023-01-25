use crate::{
    utils::base64,
    wstring::{FromWidechar, ToWidechar, LPCWSTR},
};
use arc4::Arc4;

#[no_mangle]
pub unsafe extern "stdcall" fn rc4(data_ptr: LPCWSTR, key_ptr: LPCWSTR) -> LPCWSTR {
    let data = String::from_widechar_ptr(data_ptr);
    let mut data = base64::decode(data)?;

    let key = String::from_widechar_ptr(key_ptr);
    let key = base64::decode(key)?;

    let mut cipher = Arc4::with_key(&key);
    cipher.encrypt(&mut data);

    base64::encode(data).as_widechar_ptr()
}
