use std::mem;

use crate::{string_to_ptr, unwrap_or_err, utils::{array::array_as_json, cstring}};
use serde_json::Value;

#[no_mangle]
pub extern "stdcall" fn from_list(list_ptr: *const u16) -> *const u16 {
    let list = cstring::from_ptr(list_ptr).unwrap();
    let splitted: Vec<&str> = list.split("\r\n").collect();
    let json = array_as_json( splitted);

    string_to_ptr!(&json)
}

#[no_mangle]
pub extern "stdcall" fn to_list(array_ptr: *const u16) -> *const u16 {
    let array = cstring::from_ptr(array_ptr).unwrap();
    let v: Value = unwrap_or_err!(serde_json::from_str(&array));
    let list = v.as_array()
                        .unwrap_or(&Vec::new())
                        .iter()
                        .map(|element| element.as_str().unwrap_or_default())
                        .collect::<Vec<&str>>()
                        .join("\r\n");

    string_to_ptr!(&list)
}