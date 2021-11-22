#[macro_export]
macro_rules! base64_decode_with_error {
    ( $e:expr ) => {
        match base64::decode($e) {
            Ok(decoded) => decoded,
            Err(error) => {
                let mut err_string = error.to_string();
                err_string.insert_str(0, crate::ERR);
                let wstring = cstring::to_widechar(&err_string);
                return mem::ManuallyDrop::new(wstring).as_ptr();
            },
        }
    }
}