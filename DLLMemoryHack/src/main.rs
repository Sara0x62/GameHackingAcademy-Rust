use inject_lib::Injector;
use inject_lib::Inject;
use std::path::Path;

fn main() {
    let get_pid = Injector::find_pid(inject_lib::Data::Str("wesnoth.exe")).unwrap();

    println!("get_pid: {:?}", get_pid);

    let path = Path::new("C:\\Users\\IEUser\\Desktop\\GameHacking-Academy\\DLLMemoryHack\\target\\i686-pc-windows-msvc\\debug\\dll_memhack.dll");
    let dll_file = inject_lib::Data::Path(path);
    
    println!("DLL PATH: {:?}", dll_file);

    let pid = get_pid.into_iter().nth(0).unwrap();

    println!("PID: {}", pid);

    // TODO - Fix injector - currently crashes the game.
    let inject = Injector::new(dll_file, pid);
    let res = inject.inject(false);
    let result = res.inject();
    println!("Res: {:?}", result);

    println!("Injected!");
}