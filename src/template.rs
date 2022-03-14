use crate::{cstring, debug, unwrap_or_err, DEBUG, utils};
use winapi::um::winnt::LPCWSTR;

#[no_mangle]
pub extern "stdcall" fn test(arg1: LPCWSTR) -> LPCWSTR {
    let arg1 = cstring::from_widechar_ptr(arg1);
    cstring::to_widechar_ptr(arg1+"0")
}
