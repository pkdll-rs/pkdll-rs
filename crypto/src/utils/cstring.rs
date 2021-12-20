use std::iter::once;
use std::ffi::OsStr;
pub use std::ffi::OsString;
use std::mem;
use std::os::windows::ffi::OsStrExt;
use std::os::windows::ffi::OsStringExt;

use winapi::um::winnt::PWSTR;

pub fn to_widechar_ptr<S: AsRef<OsStr> + ?Sized>(s: &S)-> PWSTR {
    let wstring: Vec<u16> = OsStr::new(s).encode_wide().chain(once(0)).collect();
    mem::ManuallyDrop::new(wstring).as_ptr() as PWSTR
}

pub fn from_widechar_ptr(data_ptr: PWSTR) -> String {
    unsafe {
        let len = (0..).take_while(|&i| *data_ptr.offset(i)!=0).count();
        let slice = std::slice::from_raw_parts(data_ptr, len);
        return OsString::from_wide(slice).into_string().unwrap();
    }
}

