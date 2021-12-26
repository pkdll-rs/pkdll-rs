use std::{time::Duration, io::{Write, Read, BufRead, BufReader}};

use uuid::Uuid;
use winapi::um::winnt::LPCWSTR;

use crate::{cstring, unwrap_or_err, proxy::Proxy, utils::tcp, CACHE};

#[no_mangle]
pub extern "stdcall" fn connect_ip(addr_ptr: LPCWSTR, proxy_addr_ptr: LPCWSTR, timeout_ptr: LPCWSTR) -> LPCWSTR {
    let addr = cstring::from_widechar_ptr(addr_ptr);
    let proxy_addr = cstring::from_widechar_ptr(proxy_addr_ptr);
    let timeout = cstring::from_widechar_ptr(timeout_ptr);

    let timeout: u64 = unwrap_or_err!(timeout.parse());
    let timeout = match timeout {
        0 => None,
        _ => Some(Duration::from_millis(timeout)),
    };

    let mut proxy: Option<Proxy> = None;

    if proxy_addr != ":" {
        proxy = Some(unwrap_or_err!(Proxy::from_pk_str(&proxy_addr)));
    }

    let stream = unwrap_or_err!(tcp::connect(addr, proxy, timeout));

    stream.set_read_timeout(Some(Duration::from_secs(5)));
    stream.set_write_timeout(Some(Duration::from_secs(5)));

    let uuid = Uuid::new_v4().to_hyphenated().to_string();

    CACHE.lock().unwrap().insert(uuid.clone(), stream);

    cstring::to_widechar_ptr(uuid)
}

#[no_mangle]
pub extern "stdcall" fn send_data(uuid_ptr: LPCWSTR, data_ptr: LPCWSTR, timeout_ptr: LPCWSTR) -> LPCWSTR {
    let uuid = cstring::from_widechar_ptr(uuid_ptr);
    let data = cstring::from_widechar_ptr(data_ptr);
    let data = unwrap_or_err!(base64::decode(data));
    let timeout = cstring::from_widechar_ptr(timeout_ptr);

    let mutex = CACHE.lock().unwrap();
    let mut stream = match mutex.get(&uuid) {
        Some(stream) => stream,
        None => {
            let mut err_string = "connection not found by given uuid".to_string();
            err_string.insert_str(0, crate::ERR);
            return cstring::to_widechar_ptr(err_string);
        }
    };

    if let Ok(timeout) = timeout.parse::<u64>() {
        stream.set_write_timeout(Some(Duration::from_millis(timeout)));
    }

    unwrap_or_err!(stream.write_all(&data));

    cstring::to_widechar_ptr("OK")
}

#[no_mangle]
pub extern "stdcall" fn recv_exact(uuid_ptr: LPCWSTR, len_ptr: LPCWSTR, timeout_ptr: LPCWSTR) -> LPCWSTR {
    let uuid = cstring::from_widechar_ptr(uuid_ptr);
    let len = cstring::from_widechar_ptr(len_ptr);
    let timeout = cstring::from_widechar_ptr(timeout_ptr);

    let len: usize = unwrap_or_err!(len.parse());

    let mutex = CACHE.lock().unwrap();
    let mut stream = match mutex.get(&uuid) {
        Some(stream) => stream,
        None => {
            let mut err_string = "connection not found by given uuid".to_string();
            err_string.insert_str(0, crate::ERR);
            return cstring::to_widechar_ptr(err_string);
        }
    };

    if let Ok(timeout) = timeout.parse::<u64>() {
        stream.set_read_timeout(Some(Duration::from_millis(timeout)));
    }

    let mut buf = vec![0u8; len];
    stream.read_exact(&mut buf);
    cstring::to_widechar_ptr(base64::encode(buf))
}

#[no_mangle]
pub extern "stdcall" fn recv_until(uuid_ptr: LPCWSTR, until_ptr: LPCWSTR, timeout_ptr: LPCWSTR) -> LPCWSTR {
    let uuid = cstring::from_widechar_ptr(uuid_ptr);
    let until = cstring::from_widechar_ptr(until_ptr);
    let timeout = cstring::from_widechar_ptr(timeout_ptr);

    let until = unwrap_or_err!(base64::decode(until));

    let mutex = CACHE.lock().unwrap();
    let stream = match mutex.get(&uuid) {
        Some(stream) => stream,
        None => {
            let mut err_string = "connection not found by given uuid".to_string();
            err_string.insert_str(0, crate::ERR);
            return cstring::to_widechar_ptr(err_string);
        }
    };

    if let Ok(timeout) = timeout.parse::<u64>() {
        stream.set_read_timeout(Some(Duration::from_millis(timeout)));
    }

    let mut buf = Vec::new();
    BufReader::new(stream).read_until(until[0], &mut buf);
    cstring::to_widechar_ptr(base64::encode(buf))
}

#[no_mangle]
pub extern "stdcall" fn recv_end(uuid_ptr: LPCWSTR, timeout_ptr: LPCWSTR) -> LPCWSTR {
    let uuid = cstring::from_widechar_ptr(uuid_ptr);
    let timeout = cstring::from_widechar_ptr(timeout_ptr);

    let mutex = CACHE.lock().unwrap();
    let mut stream = match mutex.get(&uuid) {
        Some(stream) => stream,
        None => {
            let mut err_string = "connection not found by given uuid".to_string();
            err_string.insert_str(0, crate::ERR);
            return cstring::to_widechar_ptr(err_string);
        }
    };

    if let Ok(timeout) = timeout.parse::<u64>() {
        stream.set_read_timeout(Some(Duration::from_millis(timeout)));
    }

    let mut buf = Vec::new();
    stream.read_to_end(&mut buf);
    cstring::to_widechar_ptr(base64::encode(buf))
}