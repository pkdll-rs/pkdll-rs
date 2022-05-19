use std::ffi::OsStr;
use std::ffi::OsString;
use std::mem;
use std::os::windows::ffi::OsStrExt;
use std::os::windows::ffi::OsStringExt;

use winapi::um::winnt::LPCWSTR;

pub trait ToWidechar {
	fn encode_widechar(&self) -> LPCWSTR;
}

impl<T: AsRef<OsStr>> ToWidechar for T {
    fn encode_widechar(&self) -> LPCWSTR {
		let wstring: Vec<u16> = self.as_ref().encode_wide().chain(Some(0)).collect();
		mem::ManuallyDrop::new(wstring).as_ptr()
    }
}

pub trait FromWidechar {
    fn from_widechar(widechar_ptr: LPCWSTR) -> String;
}

impl FromWidechar for String {
    fn from_widechar(widechar_ptr: LPCWSTR) -> String {
		unsafe {
			let len = (0..).take_while(|&i| *widechar_ptr.offset(i) != 0).count();
			let slice = std::slice::from_raw_parts(widechar_ptr, len);
			OsString::from_wide(slice).into_string().unwrap()
		}
	}
}
