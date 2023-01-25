use crate::{
    imp::encryption,
    utils::base64,
    wstring::{FromWidechar, ToWidechar, LPCWSTR},
};

#[no_mangle]
pub unsafe extern "stdcall" fn xor(data_ptr: LPCWSTR, key_ptr: LPCWSTR) -> LPCWSTR {
    let data = String::from_widechar_ptr(data_ptr);
    let mut data = base64::decode(data)?;

    let key = String::from_widechar_ptr(key_ptr);

    match key.parse::<u32>() {
        Ok(key) => encryption::xor_simple(&mut data, key),
        Err(_) => {
            let key = base64::decode(key)?;
            encryption::xor(&mut data, &key)
        }
    };

    base64::encode(data).as_widechar_ptr()
}
