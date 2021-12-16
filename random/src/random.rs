use std::cmp::Ordering;
use std::mem;
use rand::prelude::SliceRandom;
use rand::{Rng, thread_rng};
use serde_json::Value;
use uuid::Uuid;

use crate::utils::cstring;
use crate::{ERR, string_to_ptr, unwrap_or_err};

#[no_mangle]
pub extern "stdcall" fn shuffle(array_ptr: *const u16) -> *const u16 {
    let array = cstring::from_ptr(array_ptr);
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
    let array = cstring::from_ptr(array_ptr);
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
                        .parse::<i64>());

    let to = unwrap_or_err!(cstring::from_ptr(to_ptr)
                        .parse::<i64>());

    match from.cmp(&to) {
        Ordering::Less => {
            let mut err = String::from(ERR);
            err.push_str("`from` can't be larger than `to`");
            return string_to_ptr!(&err);
        }

        Ordering::Equal => {
            let mut err = String::from(ERR);
            err.push_str("`from` cant't be the same as `to`");
            return string_to_ptr!(&err);
        }

        _ => {}
    }

    let mut rng  = thread_rng();
    string_to_ptr!(&rng.gen_range(from..to).to_string())
}

#[no_mangle]
pub extern "stdcall" fn rangef(from_ptr: *const u16, to_ptr: *const u16, precision_ptr: *const u16) -> *const u16 {
    let from = unwrap_or_err!(cstring::from_ptr(from_ptr)
                        .parse::<f64>());

    let to = unwrap_or_err!(cstring::from_ptr(to_ptr)
                        .parse::<f64>());

    let precision = unwrap_or_err!(cstring::from_ptr(precision_ptr)
                        .parse::<usize>());

    if from > to || from == to {
        let mut err = String::from(ERR);
        err.push_str("`from` must be larger than `to`");
        return string_to_ptr!(&err);
    }

    let mut rng  = thread_rng();
    let rnd = rng.gen_range(from..to);

    string_to_ptr!(&format!("{:.1$}", rnd, precision))
}

#[no_mangle]
pub extern "stdcall" fn uuidv4() -> *const u16 {
    let uuid = Uuid::new_v4().to_hyphenated().to_string();

    string_to_ptr!(&uuid)
}

#[no_mangle]
pub extern "stdcall" fn fill_with(string_ptr: *const u16, fill_string_ptr: *const u16, num_ptr: *const u16, index_ptr: *const u16) -> *const u16 {
    let mut string = cstring::from_ptr(string_ptr);
    let fill_string = cstring::from_ptr(fill_string_ptr);

    let num = unwrap_or_err!(cstring::from_ptr(num_ptr)
                        .parse::<usize>());

    let index = cstring::from_ptr(index_ptr).parse::<usize>().unwrap_or_default();
    if string.len() > num || index > string.len() {
        return string_to_ptr!(&string);
    }
    string.insert_str(index, fill_string
                                    .repeat(num-string.len())
                                    .as_str()
    );

    string_to_ptr!(&string)
}