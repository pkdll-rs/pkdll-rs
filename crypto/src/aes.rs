use std::mem;
use base64;

pub use aes::{Aes128, Aes192, Aes256};
pub use block_modes::{
    block_padding::{AnsiX923, Iso7816, NoPadding, Pkcs7, ZeroPadding},
    Cbc, Ecb, Pcbc,
};

use crate::utils::aes::{Aes, Len, Mode, Pad};
use crate::utils::cstring;

/// inputs, outputs in base64
#[no_mangle]
pub extern "stdcall" fn aes_encrypt(data_ptr: *const u16, key_ptr: *const u16, iv_ptr: *const u16, mode_ptr: *const u16, padding_ptr: *const u16) -> *const u16 {
    let data = cstring::from_ptr(data_ptr).unwrap();
    println!("{}", data);
    let key = cstring::from_ptr(key_ptr).unwrap();
    let iv = cstring::from_ptr(iv_ptr).unwrap();
    let mode = cstring::from_ptr(mode_ptr).unwrap();
    let padding = cstring::from_ptr(padding_ptr).unwrap();

    let data = match base64::decode(data) {
        Ok(decoded) => decoded,
        Err(error) => {
            let mut err_string = error.to_string();
            err_string.insert_str(0, crate::ERR);
            let wstring = cstring::to_widechar(&err_string);
            return mem::ManuallyDrop::new(wstring).as_ptr();
        },
    };
    println!("{:?} {}", &data, String::from_utf8_lossy(&data));

    let key = match base64::decode(key) {
        Ok(decoded) => decoded,
        Err(error) => {
            let mut err_string = error.to_string();
            err_string.insert_str(0, crate::ERR);
            let wstring = cstring::to_widechar(&err_string);
            return mem::ManuallyDrop::new(wstring).as_ptr();
        },
    };

    let iv = match base64::decode(iv) {
        Ok(decoded) => decoded,
        Err(error) => {
            let mut err_string = error.to_string();
            err_string.insert_str(0, crate::ERR);
            let wstring = cstring::to_widechar(&err_string);
            return mem::ManuallyDrop::new(wstring).as_ptr();
        },
    };

    let len = match key.len() {
        16 => Len::Aes128,
        24 => Len::Aes192,
        32 => Len::Aes256,
        _ => {
            let mut err_string = String::from(format!("unsupported key len: {}. Only 16, 24, 32 bytes", key.len()));
            err_string.insert_str(0, crate::ERR);
            let wstring = cstring::to_widechar(&err_string);
            return mem::ManuallyDrop::new(wstring).as_ptr();
        }
    };

    let padding = match padding.as_str() {
        "pkcs7" => Pad::Pkcs7,
        "zero" => Pad::ZeroPadding,
        "no_padding" => Pad::NoPadding,
        "iso7816" => Pad::Iso7816,
        "ansi_x923" => Pad::AnsiX923,
        _ => {
            let mut err_string = String::from("unsupported pad type: ");
            err_string.insert_str(0, crate::ERR);
            err_string.push_str(padding.as_str());
            let wstring = cstring::to_widechar(&err_string);
            return mem::ManuallyDrop::new(wstring).as_ptr();
        }
    };

    let mode = match mode.as_str() {
        "ecb" => Mode::Ecb,
        "cbc" => Mode::Cbc,
        _ => {
            let mut err_string = String::from("unsupported pad type: ");
            err_string.insert_str(0, crate::ERR);
            err_string.push_str(mode.as_str());
            let wstring = cstring::to_widechar(&err_string);
            return mem::ManuallyDrop::new(wstring).as_ptr();
        }
    };
    
    let cipher = Aes::new(len, mode, padding);
    let encrypted = match cipher.encrypt(key, iv, data) {
        Ok(encrypted) => encrypted,
        Err(error) => {
            let mut err_string = String::from(error.to_string());
            err_string.insert_str(0, crate::ERR);
            let wstring = cstring::to_widechar(&err_string);
            return mem::ManuallyDrop::new(wstring).as_ptr();
        }
    };

    println!("{}", base64::encode(&encrypted));

    cstring::to_widechar(base64::encode(encrypted).as_str()).as_ptr()
}
