use std::{env::{self}, ffi::CString};
use std::io;
use libc;

mod command;


// docker         run image <cmd> <params>
// go run main.go run       <cmd> <params>

fn main() {
    match env::args()
        .nth(1)
        .expect("Needs at least 1 command")
        .as_str()
    {
        "run" => run(),
        _ => panic!("bad command"),
    }
}


fn set_hostname(hostname: &str) -> Result<(), io::Error> {
    let c_hostname = CString::new(hostname).unwrap();
    let res = unsafe { libc::sethostname(c_hostname.as_ptr(), hostname.len()) };

    if res != 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}

use command::{Cmd, SysProcAttr};

fn run() {
    print_log();

    let mut cmd = Cmd::command(&env::args().nth(2).unwrap(), env::args().skip(2));
    cmd.sys_proc_attr = SysProcAttr {
        clone_flags: libc::CLONE_NEWUTS | libc::CLONE_NEWPID,
    };
    let handle = cmd.run_with_proc(|| {
        print_log();
        set_hostname("container").expect("Failed setting hostname");
    }).unwrap();

    handle.wait().expect("Wait");
}

fn print_log() {
    use std::fmt::Write;
    let mut buff = String::new();
    write!(&mut buff, "Running: [").unwrap();
    for cmd in env::args().skip(2) {
        write!(&mut buff, "{} ", cmd).unwrap();
    }
    buff.pop();
    println!("{}] as {}", buff, std::process::id());
}
