use std::mem;
use crate::utils::cstring;
use crate::utils::hmac;

use crate::unwrap_or_err;



#[no_mangle]
pub extern "stdcall" fn hmac(hash_type: *const u16, data_ptr: *const u16, key_ptr: *const u16) -> *const u16 {
    let data = cstring::from_ptr(data_ptr).unwrap();
    let data = unwrap_or_err!(base64::decode(data));

    let key = cstring::from_ptr(key_ptr).unwrap();
    let key = unwrap_or_err!(base64::decode(key));

    let hash_type = cstring::from_ptr(hash_type).unwrap();

    let hashed = unwrap_or_err!(hmac::hmac_base64(data, key, hash_type));

    mem::ManuallyDrop::new(cstring::to_widechar(&hashed)).as_ptr()
}