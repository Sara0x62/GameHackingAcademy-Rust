use user32::MessageBoxA;
use winapi::um::winuser::{MB_OK, MB_ICONINFORMATION};
use std::ffi::CString;

#[no_mangle]
extern "system" fn DllMain(_: *const u8, _: u32, _: *const u8 ) -> u32 {
    unsafe {
        MessageBoxA(
            std::ptr::null_mut(),
            CString::new("Hello from the .dll").unwrap().as_ptr(),
            CString::new("hello.dll").unwrap().as_ptr(),
            MB_OK | MB_ICONINFORMATION,
        );
    }
    1 
}