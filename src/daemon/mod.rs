#[cfg(any(target_os = "linux"))]
pub fn daemonize(nochdir: bool, noclose: bool) {
    use nix::unistd;

    match unistd::daemon(nochdir, noclose) {
        Ok(()) => {
            println!("ok, entering daemon.");
        }
        Err(err) => {
            eprintln!("daemon: {}", err);
        }
    }
}

#[cfg(any(target_os = "windows", target_os = "macos"))]
pub fn daemonize(_nochdir: bool, _noclose: bool) {}
