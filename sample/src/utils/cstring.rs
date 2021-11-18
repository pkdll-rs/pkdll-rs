use std::iter::once;
use std::ffi::OsStr;
pub use std::ffi::OsString;
use std::os::windows::ffi::OsStrExt;
use std::os::windows::ffi::OsStringExt;

pub fn string_as_widechar(s: String) -> Vec<u16> {
    OsStr::new(&s).encode_wide().chain(once(0)).collect()
}

pub fn widechar_to_string(data_ptr: *const u16) -> Result<String, OsString> {
    unsafe {
        let len = (0..).take_while(|&i| *data_ptr.offset(i)!=0).count();
        let slice = std::slice::from_raw_parts(data_ptr, len);
        return OsString::from_wide(slice).into_string();
    }
}

