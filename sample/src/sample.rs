use std::mem;

#[no_mangle]
#[export_name = "returnTheSame"]
pub extern "stdcall" fn return_the_same(data_ptr: *const u16) -> *const u16 {
    let string = crate::utils::cstring::widechar_to_string(data_ptr);

    let wstring= crate::utils::cstring::string_as_widechar(string.unwrap().as_str());

    mem::ManuallyDrop::new(wstring).as_ptr()
}