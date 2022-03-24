use std::{
    io::{BufRead, BufReader, Read, Write},
    sync::Arc,
    thread,
    time::Duration,
};

use uuid::Uuid;
use winapi::um::winnt::LPCWSTR;

use crate::{cstring, errors::DllError, proxy::Proxy, unwrap_or_err, utils::tcp, TcpThread, CACHE};

#[no_mangle]
pub extern "stdcall" fn connect_ip(
    addr_ptr: LPCWSTR,
    proxy_addr_ptr: LPCWSTR,
    timeout_ptr: LPCWSTR,
) -> LPCWSTR {
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
        proxy = Some(unwrap_or_err!(Proxy::from_pk_str(proxy_addr)));
    }

    let (flag, control) = thread_control::make_pair();

    let handler = thread::spawn(move || {
        flag.alive();
        tcp::connect(addr, proxy, timeout)
    });

    let stream = TcpThread {
        tcp_stream: None,
        join_handler_connect: Some(handler),
        join_handler_write: None,
        thread_control: control,
    };

    let uuid = Uuid::new_v4().to_hyphenated().to_string();

    let mut w = unwrap_or_err!(CACHE.write());
    w.insert(uuid.clone(), stream);

    cstring::to_widechar_ptr(uuid)
}

#[no_mangle]
pub extern "stdcall" fn connect_ip_status(uuid_ptr: LPCWSTR) -> LPCWSTR {
    let uuid = cstring::from_widechar_ptr(uuid_ptr);

    {
        let mut r = unwrap_or_err!(CACHE.write());
        let mut stream = match r.get_mut(&uuid) {
            Some(stream) => stream,
            None => return cstring::to_widechar_ptr(DllError::ConnectionNotFound.to_string()),
        };

        if !stream.thread_control.is_done() {
            return cstring::to_widechar_ptr(DllError::NotYetReady.to_string());
        }

        stream.tcp_stream = Some(unwrap_or_err!(stream
            .join_handler_connect
            .take()
            .unwrap()
            .join()
            .unwrap()));
    }
    cstring::to_widechar_ptr("CONNECTED")
}

#[no_mangle]
pub extern "stdcall" fn set_read_timeout(uuid_ptr: LPCWSTR, timeout_ptr: LPCWSTR) -> LPCWSTR {
    let uuid = cstring::from_widechar_ptr(uuid_ptr);
    let timeout = cstring::from_widechar_ptr(timeout_ptr);

    let timeout: u64 = unwrap_or_err!(timeout.parse());

    let r = unwrap_or_err!(CACHE.read());
    let stream = match r.get(&uuid) {
        Some(stream) => stream,
        None => return cstring::to_widechar_ptr(DllError::ConnectionNotFound.to_string()),
    };

    if stream.tcp_stream.is_none() {
        return cstring::to_widechar_ptr(DllError::NotYetConnected.to_string());
    }

    unwrap_or_err!(stream
        .tcp_stream
        .as_ref()
        .unwrap()
        .set_read_timeout(Some(Duration::from_millis(timeout))));
    cstring::to_widechar_ptr("OK")
}

#[no_mangle]
pub extern "stdcall" fn set_write_timeout(uuid_ptr: LPCWSTR, timeout_ptr: LPCWSTR) -> LPCWSTR {
    let uuid = cstring::from_widechar_ptr(uuid_ptr);
    let timeout = cstring::from_widechar_ptr(timeout_ptr);

    let timeout: u64 = unwrap_or_err!(timeout.parse());

    let r = unwrap_or_err!(CACHE.read());
    let stream = match r.get(&uuid) {
        Some(stream) => stream,
        None => return cstring::to_widechar_ptr(DllError::ConnectionNotFound.to_string()),
    };

    if stream.tcp_stream.is_none() {
        return cstring::to_widechar_ptr(DllError::NotYetConnected.to_string());
    }

    unwrap_or_err!(stream
        .tcp_stream
        .as_ref()
        .unwrap()
        .set_write_timeout(Some(Duration::from_millis(timeout))));
    cstring::to_widechar_ptr("OK")
}

#[no_mangle]
pub extern "stdcall" fn send_data(uuid_ptr: LPCWSTR, data_ptr: LPCWSTR) -> LPCWSTR {
    let uuid = cstring::from_widechar_ptr(uuid_ptr);
    let data = cstring::from_widechar_ptr(data_ptr);
    let data = unwrap_or_err!(base64::decode(data));

    let mut r = unwrap_or_err!(CACHE.write());
    let mut stream = match r.get_mut(&uuid) {
        Some(stream) => stream,
        None => return cstring::to_widechar_ptr(DllError::ConnectionNotFound.to_string()),
    };

    let tcp_stream = stream.tcp_stream.take().unwrap();

    let (flag, control) = thread_control::make_pair();

    let handler = thread::spawn(move || {
        flag.alive();
        tcp::send_data(tcp_stream, data)
    });

    stream.thread_control = control;
    stream.join_handler_write = Some(handler);

    cstring::to_widechar_ptr("THREAD_SPAWNED")
}

#[no_mangle]
pub extern "stdcall" fn send_data_status(uuid_ptr: LPCWSTR) -> LPCWSTR {
    let uuid = cstring::from_widechar_ptr(uuid_ptr);

    {
        let mut r = unwrap_or_err!(CACHE.write());
        let mut stream = match r.get_mut(&uuid) {
            Some(stream) => stream,
            None => return cstring::to_widechar_ptr(DllError::ConnectionNotFound.to_string()),
        };

        if !stream.thread_control.is_done() {
            return cstring::to_widechar_ptr(DllError::NotYetReady.to_string());
        }

        stream.tcp_stream = Some(unwrap_or_err!(stream
            .join_handler_write
            .take()
            .unwrap()
            .join()
            .unwrap()));
    }
    cstring::to_widechar_ptr("SENT")
}

/*

#[no_mangle]
pub extern "stdcall" fn recv_exact(uuid_ptr: LPCWSTR, len_ptr: LPCWSTR) -> LPCWSTR {
    let uuid = cstring::from_widechar_ptr(uuid_ptr);
    let len = cstring::from_widechar_ptr(len_ptr);

    let len: usize = unwrap_or_err!(len.parse());

    let r = unwrap_or_err!(CACHE.read());
    let mut stream = match r.get(&uuid) {
        Some(stream) => stream,
        None => {
            let mut err_string = "connection not found by given uuid".to_string();
            err_string.insert_str(0, crate::ERR);
            return cstring::to_widechar_ptr(err_string);
        }
    };

    let mut buf = vec![0u8; len];
    unwrap_or_err!(stream.read_exact(&mut buf));
    cstring::to_widechar_ptr(base64::encode(buf))
}

#[no_mangle]
pub extern "stdcall" fn recv_until(uuid_ptr: LPCWSTR, until_ptr: LPCWSTR) -> LPCWSTR {
    let uuid = cstring::from_widechar_ptr(uuid_ptr);
    let until = cstring::from_widechar_ptr(until_ptr);

    let until = unwrap_or_err!(base64::decode(until));

    let r = unwrap_or_err!(CACHE.read());
    let stream = match r.get(&uuid) {
        Some(stream) => stream,
        None => {
            let mut err_string = "connection not found by given uuid".to_string();
            err_string.insert_str(0, crate::ERR);
            return cstring::to_widechar_ptr(err_string);
        }
    };

    if until.len() > 1 {
        let data = unwrap_or_err!(read_until(BufReader::new(stream), &until));
        return cstring::to_widechar_ptr(base64::encode(data));
    }

    let mut buf = Vec::new();

    unwrap_or_err!(BufReader::new(stream).read_until(until[0], &mut buf));
    cstring::to_widechar_ptr(base64::encode(buf))
}

#[no_mangle]
pub extern "stdcall" fn recv_end(uuid_ptr: LPCWSTR) -> LPCWSTR {
    let uuid = cstring::from_widechar_ptr(uuid_ptr);

    let r = unwrap_or_err!(CACHE.read());
    let mut stream = match r.get(&uuid) {
        Some(stream) => stream,
        None => {
            let mut err_string = "connection not found by given uuid".to_string();
            err_string.insert_str(0, crate::ERR);
            return cstring::to_widechar_ptr(err_string);
        }
    };

    let mut buf = Vec::new();
    unwrap_or_err!(stream.read_to_end(&mut buf));
    cstring::to_widechar_ptr(base64::encode(buf))
}

#[no_mangle]
pub extern "stdcall" fn disconnect(uuid_ptr: LPCWSTR) -> LPCWSTR {
    let uuid = cstring::from_widechar_ptr(uuid_ptr);

    let mut mx = unwrap_or_err!(CACHE.write());
    match mx.remove(&uuid) {
        Some(_) => cstring::to_widechar_ptr("OK"),
        None => {
            let mut err_string = "connection not found by given uuid".to_string();
            err_string.insert_str(0, crate::ERR);
            cstring::to_widechar_ptr(err_string)
        }
    }
}

fn read_until<R: Read>(mut stream: R, delim: &[u8]) -> Result<Vec<u8>, std::io::Error> {
    let mut data = vec![];
    loop {
        let mut buf = [0u8; 1];
        let n = stream.read(&mut buf)?;
        if n != 1 {
            break;
        }
        data.push(buf[0]);
        if data.len() >= delim.len() && &data[data.len() - delim.len()..] == delim {
            return Ok(data);
        }
    }
    unimplemented!()
}
*/
