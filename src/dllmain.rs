use crate::{debug, DEBUG};
use wchar::wchz;

use winapi::shared::basetsd::DWORD32;
use winapi::shared::minwindef::{BOOL, HINSTANCE, LPVOID, TRUE};
use winapi::um::libloaderapi::DisableThreadLibraryCalls;
use winapi::um::winnt::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH, LPCWSTR};
use winapi::um::{consoleapi::AllocConsole, wincon::FreeConsole};

const AUTHOR: &[u16] = wchz!("_Skill_");
const VER: &[u16] = wchz!("0.1");
const DESC: &[u16] = wchz!("");

#[no_mangle]
pub extern "stdcall" fn info_getAuthor() -> LPCWSTR {
    AUTHOR.as_ptr()
}

#[no_mangle]
pub extern "stdcall" fn info_getVersion() -> LPCWSTR {
    VER.as_ptr()
}

#[no_mangle]
pub extern "stdcall" fn info_getDescription() -> LPCWSTR {
    DESC.as_ptr()
}

#[no_mangle]
#[allow(non_snake_case)]
extern "system" fn DllMain(h_module: HINSTANCE, dw_reason: DWORD32, _: LPVOID) -> BOOL {
    match dw_reason {
        DLL_PROCESS_ATTACH => {
            unsafe {
                // We don't need to know if PK creates new threads
                DisableThreadLibraryCalls(h_module);
                if DEBUG {
                    AllocConsole();
                    debug!("Loaded");
                }
            }
        }
        DLL_PROCESS_DETACH => {
            if DEBUG {
                unsafe {
                    FreeConsole();
                }
            }
        }
        _ => {}
    }
    TRUE
}
