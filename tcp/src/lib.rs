mod dllmain;
mod tcp;

mod utils {
    pub mod cstring;
    pub mod error;
    pub mod macros;
    pub mod proxy;
    pub mod statuses;
    pub mod tcp;
    pub mod traits;
}

use crate::dllmain::DEBUG;
use once_cell::sync::Lazy;
use threadpool::{Builder, ThreadPool};

use crate::{statuses::Task, traits::TcpThread, utils::*};
use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex, RwLock, mpsc::{self, Sender}},
    time::{Duration, Instant},
};

pub const ERR: &str = "ERR|";

/*#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;
*/
const THREAD_STACK_SIZE: usize = 256 * 1024;
const THREADS_COUNT: usize = 600;

pub static mut CLEAR_THREAD_CONTROL: Option<Sender<()>> = None;

static THREAD_POOL: Lazy<Mutex<ThreadPool>> = Lazy::new(|| {
    let pool = Builder::new()
        .num_threads(THREADS_COUNT + 1)
        .thread_stack_size(THREAD_STACK_SIZE)
        .build();
    
    let (send, recv) = mpsc::channel();
    
    pool.execute(move || {
        let cache = Arc::clone(&CACHE);
        loop {
            if let Ok(_) = recv.recv_timeout(Duration::from_secs(60)) {
                debug!("Interrupted clearing cache!");
                return;
            }
    
            cleanup(&cache);
        }
    });

    unsafe {
        CLEAR_THREAD_CONTROL = Some(send);
    }

    Mutex::new(pool)
});

static CACHE: Lazy<Arc<RwLock<BTreeMap<String, TcpThread>>>> = Lazy::new(|| {
    let cache = BTreeMap::new();
    let cache = RwLock::new(cache);

    Arc::new(cache)
});

fn cleanup(cache: &Arc<RwLock<BTreeMap<String, TcpThread>>>) {
    let mut w = cache.write().unwrap();
    w.retain(|_, v| {
        if v.ttl < Instant::now() {
            if let Some(handler) = v.join_handler.take() {
                debug!("Removing: {:?}, {:?}", v, handler);
                if !v.thread_control.is_done() {
                    debug!("Not finished");
                    return true;
                }
            }
            debug!("Removed");
            return false;
        }
        true
    });
    debug!("Cache len: {}", w.len());
}
