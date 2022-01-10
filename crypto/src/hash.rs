use winapi::um::winnt::LPCWSTR;

use crate::utils::cstring;
use crate::utils::hash;

use crate::unwrap_or_err;

#[no_mangle]
pub extern "stdcall" fn hash(hash_type: LPCWSTR, data_ptr: LPCWSTR) -> LPCWSTR {
    let data = cstring::from_widechar_ptr(data_ptr);
    let data = unwrap_or_err!(base64::decode(data));
    let hash_type = cstring::from_widechar_ptr(hash_type);

    let hashed = unwrap_or_err!(hash::make_hash(data, &hash_type));

    cstring::to_widechar_ptr(&base64::encode(hashed))
}
