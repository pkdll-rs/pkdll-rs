use rand::Rng;
use uuid::Uuid;
use winapi::um::winnt::LPCWSTR;

use crate::{unwrap_or_err, utils::cstring};

#[no_mangle]
pub extern "stdcall" fn uuidv4() -> LPCWSTR {
    let uuid = Uuid::new_v4().to_hyphenated().to_string();

    cstring::to_widechar_ptr(uuid)
}

#[no_mangle]
pub extern "stdcall" fn fill_with(
    string_ptr: LPCWSTR,
    fill_string_ptr: LPCWSTR,
    num_ptr: LPCWSTR,
    index_ptr: LPCWSTR,
) -> LPCWSTR {
    let mut string = cstring::from_widechar_ptr(string_ptr);
    let fill_string = cstring::from_widechar_ptr(fill_string_ptr);

    let num = unwrap_or_err!(cstring::from_widechar_ptr(num_ptr).parse::<usize>());

    let index = cstring::from_widechar_ptr(index_ptr)
        .parse::<usize>()
        .unwrap_or_default();
    if string.len() > num || index > string.len() {
        return cstring::to_widechar_ptr(string);
    }
    string.insert_str(index, fill_string.repeat(num - string.len()).as_str());

    cstring::to_widechar_ptr(string)
}

#[no_mangle]
pub extern "stdcall" fn rand_regex(pattern_ptr: LPCWSTR) -> LPCWSTR {
    let pattern = cstring::from_widechar_ptr(pattern_ptr);

    let mut parser = regex_syntax::ParserBuilder::new().unicode(false).build();
    let hir = unwrap_or_err!(parser.parse(&pattern));
    let gen = unwrap_or_err!(rand_regex::Regex::with_hir(hir, 100));

    let mut rng = rand::thread_rng();
    let sample: String = rng.sample(gen);

    cstring::to_widechar_ptr(sample)
}
