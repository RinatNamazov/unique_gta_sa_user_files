use winapi::shared::minwindef::{DWORD, LPVOID};
use winapi::um::memoryapi::VirtualProtect;
use winapi::um::winnt::PAGE_EXECUTE_READWRITE;

pub fn patch_pointer(address: usize, value: usize) {
    let address = address as LPVOID;
    let mut vp: DWORD = PAGE_EXECUTE_READWRITE;
    unsafe {
        VirtualProtect(address, std::mem::size_of::<usize>(), vp, &mut vp);

        *(address as *mut usize) = value;

        VirtualProtect(address, std::mem::size_of::<usize>(), vp, &mut vp);
    }
}
