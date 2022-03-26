use std::{thread, time::Duration};

use uuid::Uuid;
use winapi::um::winnt::LPCWSTR;

use crate::{
    cstring,
    error::{DllError, GlobalError},
    proxy::Proxy,
    statuses::DllStatus,
    unwrap_or_err,
    utils::tcp,
    Task, TcpThread, CACHE,
};

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
        tcp::connect(addr, proxy, timeout).map_err(GlobalError::from)
    });

    let tcp_thread = TcpThread {
        tcp_stream: None,
        join_handler: Some(handler),
        thread_control: control,
        current_task: Task::Connect,
        timeout,
    };

    let uuid = Uuid::new_v4().to_hyphenated().to_string();

    let mut w = unwrap_or_err!(CACHE.write());
    w.insert(uuid.clone(), tcp_thread);

    cstring::to_widechar_ptr(uuid)
}

#[no_mangle]
pub extern "stdcall" fn send_data(uuid_ptr: LPCWSTR, data_ptr: LPCWSTR) -> LPCWSTR {
    let uuid = cstring::from_widechar_ptr(uuid_ptr);
    if let Err(error) = tcp_stream_exists(&uuid) {
        return cstring::to_widechar_ptr(error.to_string());
    }

    let data = cstring::from_widechar_ptr(data_ptr);
    let data = unwrap_or_err!(base64::decode(data));

    let mut w = unwrap_or_err!(CACHE.write());
    let mut tcp_thread = match w.get_mut(&uuid) {
        Some(tcp_thread) => tcp_thread,
        None => return cstring::to_widechar_ptr(DllError::ConnectionNotFound.to_string()),
    };

    let tcp_stream = tcp_thread.tcp_stream.take().unwrap();

    let (flag, control) = thread_control::make_pair();

    let handler = thread::spawn(move || {
        flag.alive();
        tcp::send_data(tcp_stream, data).map_err(GlobalError::from)
    });

    tcp_thread.thread_control = control;
    tcp_thread.join_handler = Some(handler);
    tcp_thread.current_task = Task::SendData;

    cstring::to_widechar_ptr(DllStatus::ThreadSpawned.as_str())
}

#[no_mangle]
pub extern "stdcall" fn recv_exact(uuid_ptr: LPCWSTR, len_ptr: LPCWSTR) -> LPCWSTR {
    let uuid = cstring::from_widechar_ptr(uuid_ptr);
    if let Err(error) = tcp_stream_exists(&uuid) {
        return cstring::to_widechar_ptr(error.to_string());
    }

    let len = cstring::from_widechar_ptr(len_ptr);
    let len: usize = unwrap_or_err!(len.parse());

    let mut w = unwrap_or_err!(CACHE.write());
    let mut tcp_thread = match w.get_mut(&uuid) {
        Some(tcp_thread) => tcp_thread,
        None => return cstring::to_widechar_ptr(DllError::ConnectionNotFound.to_string()),
    };

    let tcp_stream = tcp_thread.tcp_stream.take().unwrap();

    let (flag, control) = thread_control::make_pair();

    let handler = thread::spawn(move || {
        flag.alive();
        tcp::read_exact(tcp_stream, len).map_err(GlobalError::from)
    });

    tcp_thread.thread_control = control;
    tcp_thread.join_handler = Some(handler);
    tcp_thread.current_task = Task::RecvExact;

    cstring::to_widechar_ptr(DllStatus::ThreadSpawned.as_str())
}

#[no_mangle]
pub extern "stdcall" fn recv_until(uuid_ptr: LPCWSTR, until_ptr: LPCWSTR) -> LPCWSTR {
    let uuid = cstring::from_widechar_ptr(uuid_ptr);
    if let Err(error) = tcp_stream_exists(&uuid) {
        return cstring::to_widechar_ptr(error.to_string());
    }

    let until = cstring::from_widechar_ptr(until_ptr);
    let until = unwrap_or_err!(base64::decode(until));

    let mut w = unwrap_or_err!(CACHE.write());
    let mut tcp_thread = match w.get_mut(&uuid) {
        Some(tcp_thread) => tcp_thread,
        None => return cstring::to_widechar_ptr(DllError::ConnectionNotFound.to_string()),
    };

    let tcp_stream = tcp_thread.tcp_stream.take().unwrap();

    let (flag, control) = thread_control::make_pair();

    let handler = thread::spawn(move || {
        flag.alive();
        tcp::read_until(tcp_stream, until).map_err(GlobalError::from)
    });

    tcp_thread.thread_control = control;
    tcp_thread.join_handler = Some(handler);
    tcp_thread.current_task = Task::RecvUntil;

    cstring::to_widechar_ptr(DllStatus::ThreadSpawned.as_str())
}

#[no_mangle]
pub extern "stdcall" fn recv_end(uuid_ptr: LPCWSTR) -> LPCWSTR {
    let uuid = cstring::from_widechar_ptr(uuid_ptr);
    if let Err(error) = tcp_stream_exists(&uuid) {
        return cstring::to_widechar_ptr(error.to_string());
    }

    let mut w = unwrap_or_err!(CACHE.write());
    let mut tcp_thread = match w.get_mut(&uuid) {
        Some(tcp_thread) => tcp_thread,
        None => return cstring::to_widechar_ptr(DllError::ConnectionNotFound.to_string()),
    };

    let tcp_stream = tcp_thread.tcp_stream.take().unwrap();

    let (flag, control) = thread_control::make_pair();

    let handler = thread::spawn(move || {
        flag.alive();
        tcp::read_to_end(tcp_stream).map_err(GlobalError::from)
    });

    tcp_thread.thread_control = control;
    tcp_thread.join_handler = Some(handler);
    tcp_thread.current_task = Task::RecvEnd;

    cstring::to_widechar_ptr(DllStatus::ThreadSpawned.as_str())
}

#[no_mangle]
pub extern "stdcall" fn disconnect(uuid_ptr: LPCWSTR) -> LPCWSTR {
    let uuid = cstring::from_widechar_ptr(uuid_ptr);

    let mut mx = unwrap_or_err!(CACHE.write());
    match mx.remove(&uuid) {
        Some(_) => cstring::to_widechar_ptr(DllStatus::Ok.as_str()),
        None => cstring::to_widechar_ptr(DllError::ConnectionNotFound.to_string()),
    }
}

#[no_mangle]
pub extern "stdcall" fn task_status(uuid_ptr: LPCWSTR) -> LPCWSTR {
    let uuid = cstring::from_widechar_ptr(uuid_ptr);

    if let Err(error) = is_task_running(&uuid) {
        return cstring::to_widechar_ptr(error.to_string());
    }

    let mut r = unwrap_or_err!(CACHE.write());
    let mut tcp_thread = match r.get_mut(&uuid) {
        Some(stream) => stream,
        None => return cstring::to_widechar_ptr(DllError::ConnectionNotFound.to_string()),
    };

    if !tcp_thread.thread_control.is_done() {
        return cstring::to_widechar_ptr(DllStatus::NotYetReady.as_str());
    }

    let thread_result = unwrap_or_err!(tcp_thread.join_handler.take().unwrap().join().unwrap());

    tcp_thread.tcp_stream = Some(thread_result.tcp_stream);

    match tcp_thread.current_task {
        Task::RecvExact | Task::RecvUntil | Task::RecvEnd => {
            let result = base64::encode(thread_result.buffer.unwrap());
            return cstring::to_widechar_ptr(&result);
        }

        Task::Connect => {
            #[allow(unused_must_use)]
            {
                tcp_thread
                    .tcp_stream
                    .as_ref()
                    .unwrap()
                    .set_read_timeout(tcp_thread.timeout);
                tcp_thread
                    .tcp_stream
                    .as_ref()
                    .unwrap()
                    .set_write_timeout(tcp_thread.timeout);
            }
        }
        _ => (),
    };

    cstring::to_widechar_ptr(tcp_thread.current_task.as_str())
}

#[no_mangle]
pub extern "stdcall" fn set_read_timeout(uuid_ptr: LPCWSTR, timeout_ptr: LPCWSTR) -> LPCWSTR {
    let uuid = cstring::from_widechar_ptr(uuid_ptr);
    let timeout = cstring::from_widechar_ptr(timeout_ptr);

    let timeout = match unwrap_or_err!(timeout.parse::<u64>()) {
        0 => None,
        timeout => Some(Duration::from_millis(timeout)),
    };

    let r = unwrap_or_err!(CACHE.read());
    let stream = match r.get(&uuid) {
        Some(stream) => stream,
        None => return cstring::to_widechar_ptr(DllError::ConnectionNotFound.to_string()),
    };

    if stream.tcp_stream.is_none() {
        return cstring::to_widechar_ptr(DllError::NoTcpStream.to_string());
    }

    unwrap_or_err!(stream
        .tcp_stream
        .as_ref()
        .unwrap()
        .set_read_timeout(timeout));
    cstring::to_widechar_ptr("OK")
}

#[no_mangle]
pub extern "stdcall" fn set_write_timeout(uuid_ptr: LPCWSTR, timeout_ptr: LPCWSTR) -> LPCWSTR {
    let uuid = cstring::from_widechar_ptr(uuid_ptr);
    let timeout = cstring::from_widechar_ptr(timeout_ptr);

    let timeout = match unwrap_or_err!(timeout.parse::<u64>()) {
        0 => None,
        timeout => Some(Duration::from_millis(timeout)),
    };

    let r = unwrap_or_err!(CACHE.read());
    let stream = match r.get(&uuid) {
        Some(stream) => stream,
        None => return cstring::to_widechar_ptr(DllError::ConnectionNotFound.to_string()),
    };

    if stream.tcp_stream.is_none() {
        return cstring::to_widechar_ptr(DllError::NoTcpStream.to_string());
    }

    unwrap_or_err!(stream
        .tcp_stream
        .as_ref()
        .unwrap()
        .set_write_timeout(timeout));
    cstring::to_widechar_ptr("OK")
}

fn is_task_running(uuid: &str) -> Result<(), DllError> {
    let has_join_handler = {
        let r = CACHE.read().unwrap();
        let tcp_thread = match r.get(uuid) {
            Some(stream) => stream,
            None => return Err(DllError::ConnectionNotFound),
        };

        tcp_thread.join_handler.is_some()
    };

    if !has_join_handler {
        return Err(DllError::NoTaskRunning);
    }

    Ok(())
}

fn tcp_stream_exists(uuid: &str) -> Result<(), DllError> {
    let has_tcp_stream = {
        let r = CACHE.read().unwrap();
        let tcp_thread = match r.get(uuid) {
            Some(stream) => stream,
            None => return Err(DllError::ConnectionNotFound),
        };

        tcp_thread.tcp_stream.is_some()
    };

    if !has_tcp_stream {
        return Err(DllError::NoTcpStream);
    }

    Ok(())
}
