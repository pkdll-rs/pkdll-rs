#[derive(Debug)]
pub enum Task {
    Connect,
    SendMessage,
    ReadMessage,
}

impl Task {
    pub fn as_str(&self) -> &'static str {
        match *self {
            Task::Connect => "CONNECTED",
            Task::SendMessage => "SENT",
            Task::ReadMessage => "RECEIVED",
        }
    }
}

pub enum DllStatus {
    NotYetReady,
    ThreadSpawned,
    Ok,
}

impl DllStatus {
    pub fn as_str(&self) -> &'static str {
        match *self {
            DllStatus::NotYetReady => "WAIT",
            DllStatus::ThreadSpawned => "THREAD_SPAWNED",
            DllStatus::Ok => "OK",
        }
    }
}
