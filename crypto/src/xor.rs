use winapi::um::winnt::LPCWSTR;

use crate::utils::cstring;

use crate::unwrap_or_err;
use crate::utils::xor;

#[no_mangle]
pub extern "stdcall" fn xor(data_ptr: LPCWSTR, key_ptr: LPCWSTR) -> LPCWSTR {
    let data = cstring::from_widechar_ptr(data_ptr);
    let mut data = unwrap_or_err!(base64::decode(data));

    let key = cstring::from_widechar_ptr(key_ptr);

    match key.parse::<u32>() {
        Ok(key) => xor::xor_simple(&mut data, key),
        Err(_) => {
            let key = unwrap_or_err!(base64::decode(key));
            xor::xor(&mut data, key)
        }
    };

    cstring::to_widechar_ptr(&base64::encode(data))
}
