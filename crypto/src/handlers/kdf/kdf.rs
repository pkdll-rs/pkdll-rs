use bcrypt::BcryptError;

use crate::{
    imp::kdf::{self, error::KdfError},
    utils::base64,
    wstring::{FromWidechar, ToWidechar, LPCWSTR},
};

#[no_mangle]
pub unsafe extern "stdcall" fn bcrypt(
    data_ptr: LPCWSTR,
    cost_ptr: LPCWSTR,
    salt_ptr: LPCWSTR,
) -> LPCWSTR {
    let data = String::from_widechar_ptr(data_ptr);
    let data = base64::decode(data)?;

    let cost = String::from_widechar_ptr(cost_ptr);
    let cost = cost.parse::<u32>()?;

    let salt = String::from_widechar_ptr(salt_ptr);
    let salt = match base64::decode(&salt) {
        Ok(salt) => salt,
        Err(_) => base64::decode_bcrypt(salt)?,
    };

    let salt_len = salt.len();

    let salt: [u8; 16] = salt
        .try_into()
        .or(Err(KdfError::Bcrypt(BcryptError::InvalidSaltLen(salt_len))))?;

    let hashed = kdf::bcrypt(data, cost, salt)?;
    hashed.as_widechar_ptr()
}

/// Recommended values sufficient for most use-cases
/// - `log_n = 15` (`n = 32768`)
/// - `r = 8`
/// - `p = 1`
/// - `len = 32`
/// - `len(salt) = 16 bytes (max - 63)`
#[no_mangle]
pub unsafe extern "stdcall" fn scrypt(
    data_ptr: LPCWSTR,
    log_n_ptr: LPCWSTR,
    r_ptr: LPCWSTR,
    p_ptr: LPCWSTR,
    len_ptr: LPCWSTR,
    salt_ptr: LPCWSTR,
) -> LPCWSTR {
    let data = String::from_widechar_ptr(data_ptr);
    let data = base64::decode(data)?;

    let log_n = String::from_widechar_ptr(log_n_ptr);
    let log_n = log_n.parse::<u8>()?;

    let r = String::from_widechar_ptr(r_ptr);
    let r = r.parse::<u32>()?;

    let p = String::from_widechar_ptr(p_ptr);
    let p = p.parse::<u32>()?;

    let len = String::from_widechar_ptr(len_ptr);
    let len = len.parse::<usize>()?;

    let salt = String::from_widechar_ptr(salt_ptr);
    let salt = base64::decode(salt)?;

    let hashed = kdf::scrypt(&data, log_n, r, p, len, &salt)?;

    hashed.as_widechar_ptr()
}

#[no_mangle]
pub unsafe extern "stdcall" fn pbkdf2(
    data_ptr: LPCWSTR,
    salt_ptr: LPCWSTR,
    rounds_ptr: LPCWSTR,
    len_ptr: LPCWSTR,
    hash_type_ptr: LPCWSTR,
) -> LPCWSTR {
    let data = String::from_widechar_ptr(data_ptr);
    let data = base64::decode(data)?;

    let salt = String::from_widechar_ptr(salt_ptr);
    let salt = base64::decode(salt)?;

    let rounds = String::from_widechar_ptr(rounds_ptr);
    let rounds = rounds.parse::<u32>()?;

    let len = String::from_widechar_ptr(len_ptr);
    let len = len.parse::<usize>()?;

    let hash_type = String::from_widechar_ptr(hash_type_ptr);

    let hashed = kdf::pbkdf2(&data, &salt, rounds, len, &hash_type)?;

    base64::encode(hashed).as_widechar_ptr()
}

#[no_mangle]
pub unsafe extern "stdcall" fn evpkdf(
    data_ptr: LPCWSTR,
    salt_ptr: LPCWSTR,
    rounds_ptr: LPCWSTR,
    output_len_ptr: LPCWSTR,
    hash_type_ptr: LPCWSTR,
) -> LPCWSTR {
    let data = String::from_widechar_ptr(data_ptr);
    let data = base64::decode(data)?;

    let salt = String::from_widechar_ptr(salt_ptr);
    let salt = base64::decode(salt)?;

    let rounds = String::from_widechar_ptr(rounds_ptr);
    let rounds = rounds.parse::<usize>()?;

    let len = String::from_widechar_ptr(output_len_ptr);
    let len = len.parse::<usize>()?;

    let hash_type = String::from_widechar_ptr(hash_type_ptr);

    let hashed = kdf::evpkdf(&data, &salt, rounds, len, &hash_type)?;

    base64::encode(hashed).as_widechar_ptr()
}
