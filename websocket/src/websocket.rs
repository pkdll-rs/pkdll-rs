use crossbeam_channel::{Receiver as _Receiver, Sender as _Sender};
use url::Url;
use std::ptr;
use std::time::{Duration, Instant};

use crossbeam_channel::bounded;
use uuid::Uuid;
use winapi::um::winnt::LPCWSTR;

use crate::debug;
use crate::{
    cstring,
    error::{DllError, GlobalError},
    proxy::Proxy,
    statuses::DllStatus,
    traits::ThreadResult,
    unwrap_or_err,
    utils::websocket,
    Task, TcpThread, CACHE, DEBUG, THREAD_POOL,
};

pub const TTL: Duration = Duration::from_secs(30);

type Sender = _Sender<Result<ThreadResult, GlobalError>>;
type Receiver = _Receiver<Result<ThreadResult, GlobalError>>;

#[no_mangle]
pub extern "stdcall" fn connect_ip(
    url_ptr: LPCWSTR,
    proxy_addr_ptr: LPCWSTR,
    timeout_ptr: LPCWSTR,
    proxy_resolve_ptr: LPCWSTR,
) -> LPCWSTR {
    let url = cstring::from_widechar_ptr(url_ptr);
    let url = unwrap_or_err!(Url::parse(&url));
    let proxy_addr = cstring::from_widechar_ptr(proxy_addr_ptr);
    let timeout = cstring::from_widechar_ptr(timeout_ptr);
    let proxy_resolve: bool = cstring::from_widechar_ptr(proxy_resolve_ptr)
        .parse()
        .unwrap_or_default();

    let timeout: u64 = unwrap_or_err!(timeout.parse());
    let timeout = match timeout {
        0 => None,
        _ => Some(Duration::from_millis(timeout)),
    };

    let mut proxy: Option<Proxy> = None;

    if proxy_addr != ":" {
        proxy = Some(unwrap_or_err!(Proxy::from_pk_str(proxy_addr)));
    }

    debug!(
        "[Connect] URL: {:?}, proxy: {:?}, timeout: {:?}, proxy_resolve: {}",
        url, proxy, timeout, proxy_resolve
    );

    let (flag, control) = thread_control::make_pair();
    let (sender, recv): (Sender, Receiver) = bounded(1);

    THREAD_POOL.lock().unwrap().execute(move || {
        flag.alive();
        debug!("After alive");
        let result =
            websocket::connect(url, proxy, timeout, proxy_resolve).map_err(GlobalError::from);
        debug!("After result");
        sender.send(result).expect("send failed");
    });

    debug!("After spawn");

    let tcp_thread = TcpThread {
        stream: None,
        join_handler: Some(recv),
        thread_control: control,
        current_task: Task::Connect,
        ttl: Instant::now() + TTL,
    };

    let uuid = Uuid::new_v4().to_hyphenated().to_string();

    let mut w = unwrap_or_err!(CACHE.write());
    w.insert(uuid.clone(), tcp_thread);

    debug!("Uuid: {}", uuid);
    println!("{:?}", unsafe {
        &*ptr::slice_from_raw_parts(&CACHE as *const _ as *mut u32, 100)
    });

    cstring::to_widechar_ptr(uuid)
}

#[no_mangle]
pub extern "stdcall" fn send_data(uuid_ptr: LPCWSTR, data_ptr: LPCWSTR) -> LPCWSTR {
    let uuid = cstring::from_widechar_ptr(uuid_ptr);
    if let Err(error) = stream_exists(&uuid) {
        return cstring::to_widechar_ptr(error.to_string());
    }

    let data_str = cstring::from_widechar_ptr(data_ptr);
    let data = unwrap_or_err!(base64::decode(&data_str));

    let mut w = unwrap_or_err!(CACHE.write());
    let mut tcp_thread = match w.get_mut(&uuid) {
        Some(tcp_thread) => tcp_thread,
        None => return cstring::to_widechar_ptr(DllError::ConnectionNotFound.to_string()),
    };

    tcp_thread.increase_ttl();

    let stream = tcp_thread.stream.take().unwrap();

    debug!("[Send] uuid: {}, data: {}", uuid, data_str);

    let (flag, control) = thread_control::make_pair();

    let (sender, recv): (Sender, Receiver) = bounded(1);

    THREAD_POOL.lock().unwrap().execute(move || {
        flag.alive();
        let result = tcp::send_data(stream, data).map_err(GlobalError::from);
        sender.send(result).expect("send failed");
    });

    tcp_thread.thread_control = control;
    tcp_thread.join_handler = Some(recv);
    tcp_thread.current_task = Task::SendData;

    cstring::to_widechar_ptr(DllStatus::ThreadSpawned.as_str())
}

#[no_mangle]
pub extern "stdcall" fn recv_exact(uuid_ptr: LPCWSTR, len_ptr: LPCWSTR) -> LPCWSTR {
    let uuid = cstring::from_widechar_ptr(uuid_ptr);
    if let Err(error) = stream_exists(&uuid) {
        return cstring::to_widechar_ptr(error.to_string());
    }

    let len = cstring::from_widechar_ptr(len_ptr);
    let len: usize = unwrap_or_err!(len.parse());

    let mut w = unwrap_or_err!(CACHE.write());
    let mut tcp_thread = match w.get_mut(&uuid) {
        Some(tcp_thread) => tcp_thread,
        None => return cstring::to_widechar_ptr(DllError::ConnectionNotFound.to_string()),
    };

    tcp_thread.increase_ttl();

    let stream = tcp_thread.stream.take().unwrap();

    debug!("[RecvExact] uuid: {}, len: {}", uuid, len);

    let (flag, control) = thread_control::make_pair();

    let (sender, recv): (Sender, Receiver) = bounded(1);

    THREAD_POOL.lock().unwrap().execute(move || {
        flag.alive();
        let result = tcp::read_exact(stream, len).map_err(GlobalError::from);
        sender.send(result).expect("send failed");
    });

    tcp_thread.thread_control = control;
    tcp_thread.join_handler = Some(recv);
    tcp_thread.current_task = Task::RecvExact;

    cstring::to_widechar_ptr(DllStatus::ThreadSpawned.as_str())
}

#[no_mangle]
pub extern "stdcall" fn recv_until(uuid_ptr: LPCWSTR, until_ptr: LPCWSTR) -> LPCWSTR {
    let uuid = cstring::from_widechar_ptr(uuid_ptr);
    if let Err(error) = stream_exists(&uuid) {
        return cstring::to_widechar_ptr(error.to_string());
    }

    let until_str = cstring::from_widechar_ptr(until_ptr);
    let until = unwrap_or_err!(base64::decode(&until_str));

    let mut w = unwrap_or_err!(CACHE.write());
    let mut tcp_thread = match w.get_mut(&uuid) {
        Some(tcp_thread) => tcp_thread,
        None => return cstring::to_widechar_ptr(DllError::ConnectionNotFound.to_string()),
    };

    tcp_thread.increase_ttl();

    let stream = tcp_thread.stream.take().unwrap();

    debug!("[RecvUntil] uuid: {}, until: {}", uuid, until_str);

    let (flag, control) = thread_control::make_pair();

    let (sender, recv): (Sender, Receiver) = bounded(1);

    THREAD_POOL.lock().unwrap().execute(move || {
        flag.alive();
        let result = tcp::read_until(stream, until).map_err(GlobalError::from);
        sender.send(result).expect("send failed");
    });

    tcp_thread.thread_control = control;
    tcp_thread.join_handler = Some(recv);
    tcp_thread.current_task = Task::RecvUntil;

    cstring::to_widechar_ptr(DllStatus::ThreadSpawned.as_str())
}

#[no_mangle]
pub extern "stdcall" fn recv_end(uuid_ptr: LPCWSTR) -> LPCWSTR {
    let uuid = cstring::from_widechar_ptr(uuid_ptr);
    if let Err(error) = stream_exists(&uuid) {
        return cstring::to_widechar_ptr(error.to_string());
    }

    let mut w = unwrap_or_err!(CACHE.write());
    let mut tcp_thread = match w.get_mut(&uuid) {
        Some(tcp_thread) => tcp_thread,
        None => return cstring::to_widechar_ptr(DllError::ConnectionNotFound.to_string()),
    };

    tcp_thread.increase_ttl();

    let stream = tcp_thread.stream.take().unwrap();

    let (flag, control) = thread_control::make_pair();

    debug!("[RecvEnd] uuid: {}", uuid);

    let (sender, recv): (Sender, Receiver) = bounded(1);

    THREAD_POOL.lock().unwrap().execute(move || {
        flag.alive();
        let result = tcp::read_to_end(stream).map_err(GlobalError::from);
        sender.send(result).expect("send failed");
    });

    tcp_thread.thread_control = control;
    tcp_thread.join_handler = Some(recv);
    tcp_thread.current_task = Task::RecvEnd;

    cstring::to_widechar_ptr(DllStatus::ThreadSpawned.as_str())
}

#[no_mangle]
pub extern "stdcall" fn disconnect(uuid_ptr: LPCWSTR) -> LPCWSTR {
    let uuid = cstring::from_widechar_ptr(uuid_ptr);

    debug!("[Disconnect] uuid: {}", uuid);

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

    tcp_thread.increase_ttl();

    let thread_result = unwrap_or_err!(tcp_thread.join_handler.take().unwrap().recv().unwrap());

    tcp_thread.stream = Some(thread_result.stream);

    debug!("[TaskStatus] uuid: {}, tcp_thread: {:?}", uuid, tcp_thread);

    match tcp_thread.current_task {
        Task::RecvExact | Task::RecvUntil | Task::RecvEnd => {
            let result = base64::encode(thread_result.buffer.unwrap());
            return cstring::to_widechar_ptr(&result);
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
    let tcp_thread = match r.get(&uuid) {
        Some(tcp_thread) => tcp_thread,
        None => return cstring::to_widechar_ptr(DllError::ConnectionNotFound.to_string()),
    };

    if tcp_thread.stream.is_none() {
        return cstring::to_widechar_ptr(DllError::NoTcpStream.to_string());
    }

    debug!("[SetReadTimeout] uuid: {}, timeout: {:?}", uuid, timeout);

    unwrap_or_err!(tcp_thread
        .stream
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

    if stream.stream.is_none() {
        return cstring::to_widechar_ptr(DllError::NoTcpStream.to_string());
    }

    debug!("[SetWriteTimeout] uuid: {}, timeout: {:?}", uuid, timeout);

    unwrap_or_err!(stream.stream.as_ref().unwrap().set_write_timeout(timeout));
    cstring::to_widechar_ptr("OK")
}

fn is_task_running(uuid: &str) -> Result<(), DllError> {
    let has_join_handler = {
        let r = CACHE.read().unwrap();
        let tcp_thread = match r.get(uuid) {
            Some(tcp_thread) => tcp_thread,
            None => return Err(DllError::ConnectionNotFound),
        };

        tcp_thread.join_handler.is_some()
    };

    if !has_join_handler {
        return Err(DllError::NoTaskRunning);
    }

    Ok(())
}

fn stream_exists(uuid: &str) -> Result<(), DllError> {
    let has_stream = {
        let r = CACHE.read().unwrap();
        let tcp_thread = match r.get(uuid) {
            Some(tcp_thread) => tcp_thread,
            None => return Err(DllError::ConnectionNotFound),
        };

        tcp_thread.stream.is_some()
    };

    if !has_stream {
        return Err(DllError::NoTcpStream);
    }

    Ok(())
}
