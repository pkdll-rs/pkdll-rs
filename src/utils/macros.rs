#[macro_export]
macro_rules! unwrap_or_err {
    ( $e:expr ) => {
        match $e {
            Ok(result) => result,
            Err(error) => {
				let err_string = format!("{}|{}", crate::ERR, error);
                return err_string.encode_widechar();
            }
        }
    };
}

#[macro_export]
macro_rules! debug {
    ( $($arg:tt)+ ) => {
        if DEBUG {
            println!($($arg)+);
        }
    }
}
