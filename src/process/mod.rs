use std::ffi::OsStr;
use std::path::PathBuf;
use std::process::Command;
use sysinfo::{ProcessExt, System, SystemExt};

pub fn find_process_by_name(cmd_name: &str) -> bool {
    let s = System::new_all();
    s.processes_by_name(cmd_name).count() > 0
}

pub fn restart_process(exe_path: &str, exe_name: &str) -> bool {
    let mut path = PathBuf::new();
    path.push(exe_path);
    path.push(exe_name);

    let exe = path.as_os_str();

    let mut command = Command::new(exe);
    let mut child = command.spawn().unwrap();
    match child.try_wait() {
        Ok(Some(status)) => println!("exited with:{}", status),
        Ok(None) => {
            println!("status not ready yet, let's really wait");
            let res = child.wait();
            println!("result:{:?}", res);
        }
        Err(e) => println!("error attempting to wait:{}", e),
    }
    true
}
