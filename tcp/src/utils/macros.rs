#[macro_export]
macro_rules! unwrap_or_err {
    ( $e:expr ) => {
        match $e {
            Ok(result) => result,
            Err(error) => {
                let mut err_string = error.to_string();
                err_string.insert_str(0, crate::ERR);
                return cstring::to_widechar_ptr(err_string);
            },
        }
    }
}