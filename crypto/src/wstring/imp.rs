use std::convert;
use std::ffi::OsStr;
use std::ffi::OsString;
use std::fmt::Display;
use std::mem;
use std::ops::FromResidual;
use std::os::windows::ffi::OsStrExt;
use std::os::windows::ffi::OsStringExt;

use crate::ERR;

#[allow(clippy::upper_case_acronyms)]
#[repr(transparent)]
pub struct LPCWSTR(winapi::um::winnt::LPCWSTR);

impl<E: Display> FromResidual<Result<convert::Infallible, E>> for LPCWSTR {
    fn from_residual(residual: Result<convert::Infallible, E>) -> Self {
        match residual {
            Err(e) => {
                let mut e = e.to_string();
                e.insert_str(0, ERR);
                e.as_widechar_ptr()
            }
        }
    }
}

impl From<winapi::um::winnt::LPCWSTR> for LPCWSTR {
    fn from(value: winapi::um::winnt::LPCWSTR) -> Self {
        Self(value)
    }
}

pub trait ToWidechar {
    fn as_widechar_ptr(&self) -> LPCWSTR;
}

impl<T: AsRef<OsStr>> ToWidechar for T {
    fn as_widechar_ptr(&self) -> LPCWSTR {
        let wstring: Vec<u16> = self.as_ref().encode_wide().chain(Some(0)).collect();
        mem::ManuallyDrop::new(wstring).as_ptr().into()
    }
}

pub trait FromWidechar {
    unsafe fn from_widechar_ptr(widechar_ptr: LPCWSTR) -> String;
}

impl FromWidechar for String {
    unsafe fn from_widechar_ptr(widechar_ptr: LPCWSTR) -> String {
        let len = (0..)
            .take_while(|&i| *widechar_ptr.0.offset(i) != 0)
            .count();
        let slice = std::slice::from_raw_parts(widechar_ptr.0, len);
        OsString::from_wide(slice).into_string().unwrap()
    }
}
