use winapi::um::winnt::PWSTR;

use crate::utils::cstring;
use crate::utils::kdf;

use crate::unwrap_or_err;

#[no_mangle]
pub extern "stdcall" fn bcrypt(data_ptr: PWSTR, cost_ptr: PWSTR, salt_ptr: PWSTR) -> PWSTR {
    let data = cstring::from_ptr(data_ptr);
    let data = unwrap_or_err!(base64::decode(data));

    let cost = cstring::from_ptr(cost_ptr);
    let cost = unwrap_or_err!(cost.parse::<u32>());

    let salt = cstring::from_ptr(salt_ptr);
    let salt = unwrap_or_err!(base64::decode(salt));

    if salt.len() != 16 {
        let mut err_string = String::from("bcrypt accepts only 16 byte length salt");
        err_string.insert_str(0, crate::ERR);
        return cstring::to_widechar_ptr(&err_string);
    }

    let hashed = unwrap_or_err!(kdf::bcrypt(data, cost, salt));

    cstring::to_widechar_ptr(&hashed)
}

/// Recommended values sufficient for most use-cases
/// - `log_n = 15` (`n = 32768`)
/// - `r = 8`
/// - `p = 1`
/// - `len = 32`
/// - `len(salt) = 16 bytes (max - 63)`
#[no_mangle]
pub extern "stdcall" fn scrypt(data_ptr: PWSTR, log_n_ptr: PWSTR, r_ptr: PWSTR, p_ptr: PWSTR, len_ptr: PWSTR, salt_ptr: PWSTR) -> PWSTR {
    let data = cstring::from_ptr(data_ptr);
    let data = unwrap_or_err!(base64::decode(data));

    let log_n = cstring::from_ptr(log_n_ptr);
    let log_n = unwrap_or_err!(log_n.parse::<u8>());

    let r = cstring::from_ptr(r_ptr);
    let r = unwrap_or_err!(r.parse::<u32>());

    let p = cstring::from_ptr(p_ptr);
    let p = unwrap_or_err!(p.parse::<u32>());

    let len = cstring::from_ptr(len_ptr);
    let len = unwrap_or_err!(len.parse::<usize>());

    let salt = cstring::from_ptr(salt_ptr);
    let salt = unwrap_or_err!(base64::decode(salt));

    let hashed = unwrap_or_err!(kdf::scrypt(data, log_n, r, p, len, salt));

    cstring::to_widechar_ptr(&hashed)
}

#[no_mangle]
pub extern "stdcall" fn pbkdf2(data_ptr: PWSTR, salt_ptr: PWSTR, rounds_ptr: PWSTR, len_ptr: PWSTR, hash_type_ptr: PWSTR) -> PWSTR {
    let data = cstring::from_ptr(data_ptr);
    let data = unwrap_or_err!(base64::decode(data));

    let salt = cstring::from_ptr(salt_ptr);
    let salt = unwrap_or_err!(base64::decode(salt));

    let rounds = cstring::from_ptr(rounds_ptr);
    let rounds = unwrap_or_err!(rounds.parse::<u32>());

    let len = cstring::from_ptr(len_ptr);
    let len = unwrap_or_err!(len.parse::<usize>());

    let hash_type = cstring::from_ptr(hash_type_ptr);

    let hashed = unwrap_or_err!(kdf::pbkdf2(data, salt, rounds, len, hash_type));

    cstring::to_widechar_ptr(&hashed)
}