use std::{env, thread};
use std::fs::OpenOptions;
use std::time::Duration;

use gag::Redirect;
use wchar::wchz;

use winapi::shared::minwindef::{BOOL, DWORD, HINSTANCE, LPVOID, TRUE};
use winapi::um::libloaderapi::DisableThreadLibraryCalls;
use winapi::um::winnt::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH, LPCWSTR};
use winapi::um::{consoleapi::AllocConsole, wincon::FreeConsole};

use crate::{CLEAR_THREAD_CONTROL, debug};

const AUTHOR: &[u16] = wchz!("_Skill_");
const VER: &[u16] = wchz!("0.2");
const DESC: &[u16] = wchz!("TCP с поддержкой прокси");
pub const DEBUG: bool = false;

#[no_mangle]
extern "stdcall" fn info_getAuthor() -> LPCWSTR {
    AUTHOR.as_ptr()
}

#[no_mangle]
extern "stdcall" fn info_getVersion() -> LPCWSTR {
    VER.as_ptr()
}

#[no_mangle]
extern "stdcall" fn info_getDescription() -> LPCWSTR {
    DESC.as_ptr()
}

#[no_mangle]
#[allow(non_snake_case)]
extern "stdcall" fn DllMain(h_module: HINSTANCE, dw_reason: DWORD, _: LPVOID) -> BOOL {
    match dw_reason {
        DLL_PROCESS_ATTACH => {
            env::set_var("RUST_BACKTRACE", "full");
            unsafe {
                // We don't need to know if PK creates new threads
                DisableThreadLibraryCalls(h_module);
                if DEBUG {
                    AllocConsole();
                }
            }

            let log = OpenOptions::new()
                .append(true)
                .read(true)
                .create(true)
                .open("./panic_tcp.log")
                .unwrap();

            std::mem::forget(Redirect::stderr(log).unwrap());
        }
        DLL_PROCESS_DETACH => {
            unsafe {
                if let Some(send) = CLEAR_THREAD_CONTROL.take() {
                    send.send(()).expect("couldn't send to interrupt sleep");
                    debug!("Sent signal to stop clearing cache")
                };

                if DEBUG {
                    thread::sleep(Duration::from_secs(5));
                    FreeConsole();
                }
            }
        }
        _ => {}
    }
    TRUE
}
