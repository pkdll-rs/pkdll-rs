use std::ffi::OsStr;
use std::ffi::OsString;
use std::mem;
use std::os::windows::ffi::OsStrExt;
use std::os::windows::ffi::OsStringExt;

use winapi::um::winnt::LPCWSTR;

pub fn to_widechar_ptr<S: AsRef<OsStr>>(s: S)-> LPCWSTR {
    let wstring: Vec<u16> = s.as_ref().encode_wide().chain(Some(0)).collect();
    mem::ManuallyDrop::new(wstring).as_ptr()
}

pub fn from_widechar_ptr(data_ptr: LPCWSTR) -> String {
    unsafe {
        let len = (0..).take_while(|&i| *data_ptr.offset(i)!=0).count();
        let slice = std::slice::from_raw_parts(data_ptr, len);
        return OsString::from_wide(slice).into_string().unwrap();
    }
}

