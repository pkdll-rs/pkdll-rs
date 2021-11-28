#[macro_export]
macro_rules! unwrap_or_err {
    ( $e:expr ) => {
        match $e {
            Ok(result) => result,
            Err(error) => {
                let mut err_string = error.to_string();
                err_string.insert_str(0, crate::ERR);
                let wstring = cstring::to_widechar(&err_string);
                return mem::ManuallyDrop::new(wstring).as_ptr();
            },
        }
    }
}

#[macro_export]
macro_rules! string_to_ptr {
    ( $e:expr ) => {
        mem::ManuallyDrop::new(cstring::to_widechar($e)).as_ptr()
    }
}