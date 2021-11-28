use std::mem;
use rand::prelude::SliceRandom;
use rand::{Rng, thread_rng};
use serde_json::Value;
use uuid::Uuid;

use crate::utils::cstring;
use crate::{ERR, string_to_ptr, unwrap_or_err};

#[no_mangle]
pub extern "stdcall" fn shuffle(array_ptr: *const u16) -> *const u16 {
    let array = cstring::from_ptr(array_ptr).unwrap();
    let mut v: Value = unwrap_or_err!(serde_json::from_str(&array));

    let array = match v.as_array_mut() {
        Some(i) => i,
        None => {
            let mut err = String::from(ERR);
            err.push_str("can't interpret as array");
            return string_to_ptr!(ERR);
        }
    };
    let mut rng = thread_rng();
    array.shuffle(&mut rng);

    string_to_ptr!(&unwrap_or_err!(serde_json::to_string(array)))
}

#[no_mangle]
pub extern "stdcall" fn choice(array_ptr: *const u16) -> *const u16 {
    let array = cstring::from_ptr(array_ptr).unwrap();
    let v: Value = unwrap_or_err!(serde_json::from_str(&array));

    let array = match v.as_array() {
        Some(i) => i,
        None => {
            let mut err = String::from(ERR);
            err.push_str("can't interpret as array");
            return string_to_ptr!(&err);
        }
    };

    let mut rng  = thread_rng();

    string_to_ptr!(array[rng.gen_range(0..array.len())].as_str().unwrap_or_default())
}

#[no_mangle]
pub extern "stdcall" fn range(from_ptr: *const u16, to_ptr: *const u16) -> *const u16 {
    let from = unwrap_or_err!(cstring::from_ptr(from_ptr)
                        .unwrap()
                        .parse::<i64>());

    let to = unwrap_or_err!(cstring::from_ptr(to_ptr)
                        .unwrap()
                        .parse::<i64>());

    if from > to {
        let mut err = String::from(ERR);
        err.push_str("`from` can't be larger than `to`");
        return string_to_ptr!(&err);
    }

    let mut rng  = thread_rng();
    string_to_ptr!(&rng.gen_range(from..to).to_string())
}

#[no_mangle]
pub extern "stdcall" fn uuidv4() -> *const u16 {
    let uuid = Uuid::new_v4().to_hyphenated().to_string();

    string_to_ptr!(&uuid)
}