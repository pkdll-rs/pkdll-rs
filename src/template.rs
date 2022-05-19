use std::{sync::mpsc::RecvError, fs::File};

use crate::{cstring::{FromWidechar, ToWidechar}, debug, unwrap_or_err, DEBUG, utils};
use winapi::um::winnt::LPCWSTR;

#[no_mangle]
pub extern "stdcall" fn test(arg1: LPCWSTR) -> LPCWSTR {
    let arg1 = String::from_widechar(arg1);
    (arg1+"0").encode_widechar()
}
