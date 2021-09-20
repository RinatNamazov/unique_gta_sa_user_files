use winapi::shared::minwindef::{BOOL, DWORD, HMODULE, LPVOID, TRUE};
use winapi::um::libloaderapi::DisableThreadLibraryCalls;
use winapi::um::winnt::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH};

pub mod plugin;
pub mod utils;

#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn DllMain(instance: HMODULE, reason: DWORD, _reserved: LPVOID) -> BOOL {
    match reason {
        DLL_PROCESS_ATTACH => {
            unsafe {
                DisableThreadLibraryCalls(instance);
            }
            plugin::initialize();
        }
        DLL_PROCESS_DETACH => {
            plugin::uninitialize();
        }
        _ => {}
    }
    return TRUE;
}
