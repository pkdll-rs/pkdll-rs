use std::fs::OpenOptions;
use std::{thread, env};
use std::time::Duration;

use gag::Redirect;
use wchar::wchz;

use winapi::shared::minwindef::{BOOL, DWORD, HINSTANCE, LPVOID, TRUE, FALSE};
use winapi::um::libloaderapi::DisableThreadLibraryCalls;
use winapi::um::winnt::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH, LPCWSTR};
use winapi::um::{consoleapi::AllocConsole, wincon::FreeConsole};

use crate::{CACHE};

const AUTHOR: &[u16] = wchz!("_Skill_");
const VER: &[u16] = wchz!("0.1");
const DESC: &[u16] = wchz!("TCP с поддержкой прокси");
const DEBUG: bool = true;

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

            Redirect::stderr(log).unwrap();

            env::set_var("RUST_BACKTRACE", "full");
            println!("aa");

            /*
            thread::spawn(|| {
                loop {
                    thread::sleep(Duration::from_secs(10));
                    let r = CACHE.read().unwrap();
                    println!("===============================================");
                    println!("Cache: {:?}", r);
                    println!("Len: {}", r.len());
                    println!("===============================================");
                }
            });*/
        }
        DLL_PROCESS_DETACH => {
            /*if DEBUG {
                unsafe {
                    FreeConsole();
                }
            }*/

            //unsafe {A.take().unwrap().join()};
            
            /*if let Err(_) = CACHE.read() {
                return FALSE;
            }*/
        }
        _ => {}
    }
    TRUE
}
