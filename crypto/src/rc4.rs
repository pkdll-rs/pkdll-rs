use arc4::Arc4;
use winapi::um::winnt::LPCWSTR;

use crate::utils::cstring;

use crate::unwrap_or_err;

#[no_mangle]
pub extern "stdcall" fn rc4(data_ptr: LPCWSTR, key_ptr: LPCWSTR) -> LPCWSTR {
    let data = cstring::from_widechar_ptr(data_ptr);
    let mut data = unwrap_or_err!(base64::decode(data));

    let key = cstring::from_widechar_ptr(key_ptr);
    let key = unwrap_or_err!(base64::decode(key));

    let mut cipher = Arc4::with_key(&key);
    cipher.encrypt(&mut data);


    cstring::to_widechar_ptr(&base64::encode(data))
}