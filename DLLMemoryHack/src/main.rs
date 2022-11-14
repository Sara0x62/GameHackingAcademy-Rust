use dll_syringe::{Syringe, process::OwnedProcess};


fn main() {
    let proc_name = "wesnoth.exe";

    let target_proc = OwnedProcess::find_first_by_name(proc_name).unwrap();
    let injector = Syringe::for_process(target_proc);

    let payload = injector.inject("./target/release/dll_memory_hack.dll").unwrap();

}
