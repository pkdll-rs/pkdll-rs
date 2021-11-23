use std::mem;
use crate::utils::cstring;
use crate::utils::hmac;

use crate::base64_decode_with_error;

#[no_mangle]
pub extern "stdcall" fn hmac(hash_type: *const u16, data_ptr: *const u16, key_ptr: *const u16) -> *const u16 {
    let data = cstring::from_ptr(data_ptr).unwrap();
    let data = base64_decode_with_error!(data);

    let key = cstring::from_ptr(key_ptr).unwrap();
    let key = base64_decode_with_error!(key);

    let hash_type = cstring::from_ptr(hash_type).unwrap();


    let hashed = match hmac::hmac_base64(data, key, hash_type) {
        Ok(hashed) => hashed,
        Err(error) => {
            let mut err_string = error.to_string();
            err_string.insert_str(0, crate::ERR);
            let wstring = cstring::to_widechar(&err_string);
            return mem::ManuallyDrop::new(wstring).as_ptr();
        }
    };

    mem::ManuallyDrop::new(cstring::to_widechar(&hashed)).as_ptr()
}