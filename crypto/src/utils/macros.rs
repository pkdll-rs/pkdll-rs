#[macro_export]
macro_rules! debug {
    ( $($arg:tt)+ ) => {
        if $crate::DEBUG {
            println!($($arg)+);
        }
    }
}
