use winapi::um::winnt::PWSTR;

use crate::utils::cstring;
use crate::utils::random;

use crate::unwrap_or_err;

#[no_mangle]
pub extern "stdcall" fn random_bytes(len_ptr: PWSTR) -> PWSTR {
    let len = cstring::from_ptr(len_ptr);
    let len = unwrap_or_err!(len.parse::<usize>());

    let hashed = random::random_bytes(len);

    cstring::to_widechar_ptr(&base64::encode(hashed))
}