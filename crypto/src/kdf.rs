use std::mem;

use crate::utils::cstring;
use crate::utils::kdf;

use crate::unwrap_or_err;

#[no_mangle]
pub extern "stdcall" fn bcrypt(data_ptr: *const u16, cost_ptr: *const u16, salt_ptr: *const u16) -> *const u16 {
    let data = cstring::from_ptr(data_ptr).unwrap();
    let data = unwrap_or_err!(base64::decode(data));

    let cost = cstring::from_ptr(cost_ptr).unwrap();
    let cost = unwrap_or_err!(cost.parse::<u32>());

    let salt = cstring::from_ptr(salt_ptr).unwrap();
    let salt = unwrap_or_err!(base64::decode(salt));

    if salt.len() != 16 {
        let mut err_string = String::from("bcrypt accepts only 16 byte length salt");
        err_string.insert_str(0, crate::ERR);
        let wstring = cstring::to_widechar(&err_string);
        return mem::ManuallyDrop::new(wstring).as_ptr();
    }

    let hashed = unwrap_or_err!(kdf::bcrypt(data, cost, salt));

    mem::ManuallyDrop::new(cstring::to_widechar(&hashed)).as_ptr()
}

/// Recommended values sufficient for most use-cases
/// - `log_n = 15` (`n = 32768`)
/// - `r = 8`
/// - `p = 1`
/// - `len = 32`
/// - `len(salt) = 16 bytes (max - 63)`
#[no_mangle]
pub extern "stdcall" fn scrypt(data_ptr: *const u16, log_n_ptr: *const u16, r_ptr: *const u16, p_ptr: *const u16, len_ptr: *const u16, salt_ptr: *const u16) -> *const u16 {
    let data = cstring::from_ptr(data_ptr).unwrap();
    let data = unwrap_or_err!(base64::decode(data));

    let log_n = cstring::from_ptr(log_n_ptr).unwrap();
    let log_n = unwrap_or_err!(log_n.parse::<u8>());

    let r = cstring::from_ptr(r_ptr).unwrap();
    let r = unwrap_or_err!(r.parse::<u32>());

    let p = cstring::from_ptr(p_ptr).unwrap();
    let p = unwrap_or_err!(p.parse::<u32>());

    let len = cstring::from_ptr(len_ptr).unwrap();
    let len = unwrap_or_err!(len.parse::<usize>());

    let salt = cstring::from_ptr(salt_ptr).unwrap();
    let salt = salt.trim_end_matches("=");

    let hashed = unwrap_or_err!(kdf::scrypt(data, log_n, r, p, len, salt));

    mem::ManuallyDrop::new(cstring::to_widechar(&hashed)).as_ptr()
}