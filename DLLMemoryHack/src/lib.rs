use winapi::{
        um::{
        winuser::{MB_OK, MB_ICONINFORMATION, GetAsyncKeyState}, 
        winnt::DLL_PROCESS_ATTACH,
        processthreadsapi::CreateThread,
        minwinbase::{LPTHREAD_START_ROUTINE, SECURITY_ATTRIBUTES},
        
    }, 
    shared::basetsd::DWORD_PTR
};
use winapi::shared::ntdef::NULL;
use winapi::shared::minwindef::LPVOID;

// used for testing if injector worked
// use user32::MessageBoxA;
// use std::ffi::CString;

fn injected_thread() {
    loop {
        unsafe {
            if GetAsyncKeyState('M' as i32) != 0 {
                let base_addr = 0x017EED18 as *mut DWORD_PTR;
                let game_base = (*base_addr + 0xA90) as *mut DWORD_PTR;
                let gold_addr = (*game_base + 0x4) as *mut DWORD_PTR;

                *gold_addr = 9999;
            }
        }
    }
}

#[no_mangle]
extern "system" fn DllMain(_: *const u8, fwdReason: u32, _: *const u8 ) -> u32 {
    unsafe {
        if fwdReason == DLL_PROCESS_ATTACH {
            let lpthread_start = Some(*(&injected_thread() as *const _ as *const unsafe extern "system" fn(LPVOID) -> u32));
            CreateThread(0 as *mut SECURITY_ATTRIBUTES, 0, lpthread_start, NULL, 0, 0 as *mut u32);
        }
        /*
        // Uncomment to test if it's actually running
        MessageBoxA(
            std::ptr::null_mut(),
            CString::new("Hello from the .dll").unwrap().as_ptr(),
            CString::new("hello.dll").unwrap().as_ptr(),
            MB_OK | MB_ICONINFORMATION,
        );
        */
    }

    1 
}