use std::mem;

use crate::utils::cstring;
use crate::utils::bcrypt;

use crate::unwrap_or_err;

#[no_mangle]
pub extern "stdcall" fn bcrypt(data_ptr: *const u16, cost_ptr: *const u16, salt_ptr: *const u16) -> *const u16 {
    let data = cstring::from_ptr(data_ptr).unwrap();
    let data = unwrap_or_err!(base64::decode(data));

    let cost = cstring::from_ptr(cost_ptr).unwrap();
    let cost = unwrap_or_err!(cost.parse::<u32>());

    let salt = cstring::from_ptr(salt_ptr).unwrap();
    let salt = unwrap_or_err!(base64::decode(salt));

    let hashed = unwrap_or_err!(bcrypt::make_hash(data, cost, salt));

    mem::ManuallyDrop::new(cstring::to_widechar(&hashed)).as_ptr()
}