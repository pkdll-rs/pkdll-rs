use std::{
    ffi::{OsStr, OsString},
    iter::once,
    mem,
    os::windows::prelude::{OsStrExt, OsStringExt},
};

use criterion::{criterion_group, criterion_main, Criterion};
use libloading::Library;
use winapi::shared::ntdef::LPCWSTR;

pub fn to_widechar<S: AsRef<OsStr> + ?Sized>(s: &S) -> Vec<u16> {
    OsStr::new(s).encode_wide().chain(once(0)).collect()
}

pub fn from_ptr(data_ptr: *const u16) -> Result<String, OsString> {
    unsafe {
        let len = (0..).take_while(|&i| *data_ptr.offset(i) != 0).count();
        let slice = std::slice::from_raw_parts(data_ptr, len);
        OsString::from_wide(slice).into_string()
    }
}

pub fn hash_benchmark(c: &mut Criterion) {
    let func: libloading::Symbol<unsafe extern "system" fn(LPCWSTR, LPCWSTR) -> LPCWSTR>;
    let lib: Library;
    unsafe {
        lib = libloading::Library::new("./crypto.dll").expect("can't load crypto.dll");
        func = lib.get(b"hash").expect("can't find func hash");
    }

    c.bench_function("hash(\"sha256\", \"test\")", |b| {
        b.iter(|| {
            let text = mem::ManuallyDrop::new(to_widechar(&base64::encode("test")));
            let hash_type = mem::ManuallyDrop::new(to_widechar("sha512"));
            unsafe {
                func(hash_type.as_ptr() as LPCWSTR, text.as_ptr() as LPCWSTR);
            }
        })
    });
}

criterion_group!(benches, hash_benchmark);
criterion_main!(benches);
