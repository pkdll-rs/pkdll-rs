use rand::{thread_rng, Rng};
use winapi::um::winnt::LPCWSTR;

use crate::{cstring, unwrap_or_err, ERR};

#[no_mangle]
pub extern "stdcall" fn range(from_ptr: LPCWSTR, to_ptr: LPCWSTR) -> LPCWSTR {
    let from = unwrap_or_err!(cstring::from_widechar_ptr(from_ptr).parse::<i64>());

    let to = unwrap_or_err!(cstring::from_widechar_ptr(to_ptr).parse::<i64>());

    if from >= to {
        let mut err = ERR.to_string();
        err.push_str("`to` must be larger than `from`");
        return cstring::to_widechar_ptr(err);
    }

    let mut rng = thread_rng();
    cstring::to_widechar_ptr(rng.gen_range(from..to).to_string())
}

#[no_mangle]
pub extern "stdcall" fn rangef(
    from_ptr: LPCWSTR,
    to_ptr: LPCWSTR,
    precision_ptr: LPCWSTR,
) -> LPCWSTR {
    let from = unwrap_or_err!(cstring::from_widechar_ptr(from_ptr).parse::<f64>());

    let to = unwrap_or_err!(cstring::from_widechar_ptr(to_ptr).parse::<f64>());

    let precision = unwrap_or_err!(cstring::from_widechar_ptr(precision_ptr).parse::<usize>());

    if from >= to {
        let mut err = ERR.to_string();
        err.push_str("`to` must be larger than `from`");
        return cstring::to_widechar_ptr(err);
    }

    let mut rng = thread_rng();
    let rnd = rng.gen_range(from..to);

    cstring::to_widechar_ptr(format!("{:.1$}", rnd, precision))
}
