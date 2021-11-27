use std::mem;
use gjson;

use crate::utils::cstring;

#[no_mangle]
pub extern "stdcall" fn get(json_ptr: *const u16, path_ptr: *const u16) -> *const u16 {
    let json = cstring::from_ptr(json_ptr).unwrap();
    let path = cstring::from_ptr(path_ptr).unwrap();
    let value = gjson::get(&json, &path);

    mem::ManuallyDrop::new(cstring::to_widechar(value.str())).as_ptr()
}