use rand::{prelude::SliceRandom, thread_rng, Rng};
use serde_json::Value;
use winapi::um::winnt::LPCWSTR;

use crate::{cstring, unwrap_or_err, ERR};

#[no_mangle]
pub extern "stdcall" fn shuffle(array_ptr: LPCWSTR) -> LPCWSTR {
    let array = cstring::from_widechar_ptr(array_ptr);
    let mut v: Value = unwrap_or_err!(serde_json::from_str(&array));

    let array = match v.as_array_mut() {
        Some(i) => i,
        None => {
            let mut err = ERR.to_string();
            err.push_str("can't interpret as array");
            return cstring::to_widechar_ptr(err);
        }
    };
    let mut rng = thread_rng();
    array.shuffle(&mut rng);

    cstring::to_widechar_ptr(unwrap_or_err!(serde_json::to_string(array)))
}

#[no_mangle]
pub extern "stdcall" fn choice(array_ptr: LPCWSTR) -> LPCWSTR {
    let array = cstring::from_widechar_ptr(array_ptr);
    let v: Value = unwrap_or_err!(serde_json::from_str(&array));

    let array = match v.as_array() {
        Some(i) => i,
        None => {
            let mut err = ERR.to_string();
            err.push_str("can't interpret as array");
            return cstring::to_widechar_ptr(err);
        }
    };

    let mut rng = thread_rng();

    cstring::to_widechar_ptr(array[rng.gen_range(0..array.len())].to_string())
}
