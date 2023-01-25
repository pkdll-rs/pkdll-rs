use crate::{wstring::{LPCWSTR, FromWidechar, ToWidechar}, utils::base64, imp::tool};

#[no_mangle]
pub unsafe extern "stdcall" fn random_bytes(len_ptr: LPCWSTR) -> LPCWSTR {
    let len = String::from_widechar_ptr(len_ptr);
    let len = len.parse::<usize>()?;

    let hashed = tool::random::random_bytes(len);

    base64::encode(hashed).as_widechar_ptr()
}
