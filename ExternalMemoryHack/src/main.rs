use winapi::{
    // Variable types etc.
    shared::minwindef::{FALSE, LPCVOID, LPVOID},
    shared::ntdef::NULL,
    shared::windef::HWND,
    um::winnt::{HANDLE, PROCESS_ALL_ACCESS},

    // Functions
    um::memoryapi::{ReadProcessMemory, WriteProcessMemory},
    um::winuser::{FindWindowA, GetWindowThreadProcessId},
    um::processthreadsapi::OpenProcess,

    // Getting error codes
    um::errhandlingapi::GetLastError,
};
// exit, flush, input
use std::{
    process::exit,
    io::stdout,
    io::Write,
    io::stdin,
};

// Gold location:
// [[[0x17EED18] + 0xA90] + 0x4]
struct GameCheat {
    handle: HANDLE,
    gold_value: u64,
    gold_addr: u64,
}

impl GameCheat {
    fn new() -> GameCheat {

        let proc_handle: HANDLE = Self::get_proc_handle();
        let (addr, val) = Self::get_gold_addr(proc_handle);

        GameCheat {
            handle: proc_handle,
            gold_value: val,
            gold_addr: addr,
        }
    }

    fn get_proc_handle() -> HANDLE {
        // Setting up process access
        let proc_handle: HANDLE;
        unsafe {
            let title = std::ffi::CString::new("The Battle for Wesnoth - 1.14.9").unwrap();
            let window: HWND = FindWindowA(std::ptr::null_mut(), title.as_ptr());

            // FindWindowA returns NULL if no matching window is found
            if window == NULL as HWND {
                println!("Window not found! Exiting.\nIs the game open?");
                exit(0);
            }

            let mut proc_id: u32 = 0;
            GetWindowThreadProcessId(window, &mut proc_id);

            // Process ID didnt get set - still 0 - Get last error code and exit;
            if proc_id == 0 || GetLastError() != 0 {
                println!("Error, unable to get process ID | LastError = {}", GetLastError());
                exit(0)
            }

            // Process handle
            proc_handle = OpenProcess(PROCESS_ALL_ACCESS, FALSE, proc_id);

            if GetLastError() != 0 {
                println!("Error, unable to get open process! | LastError = {}", GetLastError());
                exit(0);
            }
        }

        proc_handle
    }

    fn get_gold_addr(handle: HANDLE) -> (u64, u64) {
        let gold_base: u64 = 0x017EED18;
        let mut gold_val: u64 = 0;
        let mut addr: u64 = 0;
        let mut bytes_read: usize = 0;

        unsafe {
            ReadProcessMemory(
                handle,
                gold_base as LPCVOID,
                &mut addr as *mut _ as LPVOID,
                4,
                &mut bytes_read,
            );

            addr += 0xA90;
            ReadProcessMemory(
                handle,
                addr as LPCVOID,
                &mut addr as *mut _ as LPVOID,
                4,
                &mut bytes_read,
            );

            addr += 0x4;
            ReadProcessMemory(
                handle,
                addr as LPCVOID,
                &mut gold_val as *mut _ as LPVOID,
                4,
                &mut bytes_read,
            );

            (addr, gold_val)
        }
    }

    pub fn increment_gold(&mut self, increment_by: u64) {
        let mut bytes_written: usize = 0;
        self.gold_value += increment_by;
        let mut new_gold: u64 = self.gold_value;
        println!("New gold: {:?}", self.gold_value);

        unsafe {
            WriteProcessMemory(
                self.handle,
                self.gold_addr as LPVOID,
                &mut new_gold as *mut _ as LPCVOID,
                4,
                &mut bytes_written,
            );
            println!("Error code: {}", GetLastError());
            println!("Addr: {:#x}", self.gold_addr);
        }
    }
}

fn main() {
    println!("[GameHacking.Academy - in Rust]");
    println!("Wesnoth - simple gold hack by Sara \"Sain\"");
    println!("Usage; enter a number to increment your gold by or 'q' to quit");

    let mut game = GameCheat::new();

    let mut increment_val: u64 = 0;
    let mut buffer = String::new();

    loop {
        buffer.clear();

        print!("Increment gold by: ");
        _ = stdout().flush();

        stdin().read_line(&mut buffer).expect("Error reading line");
        if buffer.trim() == "q".to_string() { exit(0); }
        match buffer.trim().parse() {
            Ok(n) => increment_val = n,
            Err(e) => println!("Invalid integer value | Error: {e}"),
        }

        game.increment_gold(increment_val);
    }
}
