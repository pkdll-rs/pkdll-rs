use std::mem;
use crate::utils::cstring;
use crate::utils::hash;

use crate::unwrap_or_err;

#[no_mangle]
pub extern "stdcall" fn hash(hash_type: *const u16, data_ptr: *const u16) -> *const u16 {
    let data = cstring::from_ptr(data_ptr).unwrap();
    let data = unwrap_or_err!(base64::decode(data));
    let hash_type = cstring::from_ptr(hash_type).unwrap();

    let hashed = unwrap_or_err!(hash::make_hash(data, &hash_type));

    mem::ManuallyDrop::new(cstring::to_widechar(&base64::encode(hashed))).as_ptr()
}