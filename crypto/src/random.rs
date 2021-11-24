use std::mem;

use crate::utils::cstring;
use crate::utils::random;

use crate::unwrap_or_err;

#[no_mangle]
pub extern "stdcall" fn random_bytes(len_ptr: *const u16) -> *const u16 {
    let len = cstring::from_ptr(len_ptr).unwrap();
    let len = unwrap_or_err!(len.parse::<usize>());

    let hashed = random::random_bytes(len);

    mem::ManuallyDrop::new(cstring::to_widechar(&base64::encode(hashed))).as_ptr()
}