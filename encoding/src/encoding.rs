use std::mem;

use base64;
use hex;

const ERR: &str = "ERR|";


#[no_mangle]
pub extern "stdcall" fn b64_encode(data_ptr: *const u16) -> *const u16 {
    let data = crate::utils::cstring::from_ptr(data_ptr).unwrap();

    let encoded_data = base64::encode(data);
    let wstring = crate::utils::cstring::to_widechar(&encoded_data);

    mem::ManuallyDrop::new(wstring).as_ptr()
}

#[no_mangle]
pub extern "stdcall" fn b64_decode(data_ptr: *const u16) -> *const u16 {
    let data = crate::utils::cstring::from_ptr(data_ptr).unwrap();

    let decoded_data = base64::decode(data).unwrap_or_else(|error| {
        let mut err_string = error.to_string();
        err_string.insert_str(0, ERR);
        err_string.into_bytes()
    });

    let wstring = crate::utils::cstring::to_widechar(&String::from_utf8_lossy(&decoded_data));

    mem::ManuallyDrop::new(wstring).as_ptr()
}

#[no_mangle]
pub extern "stdcall" fn hex_encode(data_ptr: *const u16) -> *const u16 {
    let data = crate::utils::cstring::from_ptr(data_ptr).unwrap();

    let encoded_data = hex::encode(data);

    let wstring = crate::utils::cstring::to_widechar(&encoded_data);

    mem::ManuallyDrop::new(wstring).as_ptr()
}

#[no_mangle]
pub extern "stdcall" fn hex_decode(data_ptr: *const u16) -> *const u16 {
    let data = crate::utils::cstring::from_ptr(data_ptr).unwrap();

    let decoded_data = hex::decode(data).unwrap_or_else(|error| {
        let mut err_string = error.to_string();
        err_string.insert_str(0, ERR);
        err_string.into_bytes()
    });

    let wstring = crate::utils::cstring::to_widechar(&String::from_utf8_lossy(&decoded_data));

    mem::ManuallyDrop::new(wstring).as_ptr()
}

#[no_mangle]
pub extern "stdcall" fn hex_to_b64(data_ptr: *const u16) -> *const u16 {
    let data = crate::utils::cstring::from_ptr(data_ptr).unwrap();

    let decoded_data = hex::decode(data);

    let wstring = match decoded_data {
        Ok(data) => {
            let encoded_data = base64::encode(data);
            crate::utils::cstring::to_widechar(&encoded_data)
        }

        Err(error) => {
            let mut err_string = error.to_string();
            err_string.insert_str(0, ERR);
            crate::utils::cstring::to_widechar(&err_string)
        }
    };

    mem::ManuallyDrop::new(wstring).as_ptr()
}

#[no_mangle]
pub extern "stdcall" fn b64_to_hex(data_ptr: *const u16) -> *const u16 {
    let data = crate::utils::cstring::from_ptr(data_ptr).unwrap();

    let decoded_data = base64::decode(data);


    let wstring = match decoded_data {
        Ok(data) => {
            let encoded_data = hex::encode(data);
            crate::utils::cstring::to_widechar(&encoded_data)
        }

        Err(error) => {
            let mut err_string = error.to_string();
            err_string.insert_str(0, ERR);
            crate::utils::cstring::to_widechar(&err_string)
        }
    };

    mem::ManuallyDrop::new(wstring).as_ptr()
}