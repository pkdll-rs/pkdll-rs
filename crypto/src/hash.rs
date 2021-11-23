use std::mem;
use crate::utils::cstring;
use crate::utils::hash;

use crate::unwrap_or_err;

#[no_mangle]
pub extern "stdcall" fn hash(hash_type: *const u16, data_ptr: *const u16) -> *const u16 {
    let data = cstring::from_ptr(data_ptr).unwrap();
    let data = unwrap_or_err!(base64::decode(data));
    let hash_type = cstring::from_ptr(hash_type).unwrap();

    let hashed = match hash::hash_base64(data, hash_type) {
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