use std::{env::{self}, ffi::CString};
use std::{io, ptr};
use libc::{self, c_char};


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


struct CmdHandle {
    handle: i32,
}

impl CmdHandle {
    pub fn wait(self) -> Result<(), io::Error> {
        let wait_res = unsafe { libc::waitpid(self.handle, ptr::null_mut(), libc::__WALL) };

        if wait_res == -1 {
            Err(std::io::Error::last_os_error())
        } else {
            Ok(())
        }
    }
}

struct Cmd {
    cmd: CString,
    pub sys_proc_attr: SysProcAttr,
    args: Vec<CString>,
}

#[derive(Default)]
struct SysProcAttr {
    clone_flags: i32,
}

impl Cmd {
    pub fn command<A>(cmd: &str, args: A) -> Self
    where A: IntoIterator<Item = String>,
    {
        let args: Vec<CString> = args.into_iter().map(|a| CString::new(a.as_str()).unwrap_or_default()).collect();
        Self {
            cmd: CString::new(cmd).expect("Invalid command"),
            sys_proc_attr: SysProcAttr::default(),
            args,
        }
    }

    pub fn run(self) -> Result<CmdHandle, io::Error> {
        self.run_proc(|| {
            let mut args: Vec<*const c_char> = self.args.iter().map(|a| a.as_ptr()).collect();
            args.push(ptr::null());

            //let cmd = CString::new(cmd).expect("Failed cstring conversion");
            let res = unsafe { libc::execvp(self.cmd.as_ptr(), args.as_ptr()) };

            if res == -1 {
                panic!("execl err: {}", io::Error::last_os_error());
            }
        })
    }

    pub fn run_proc<F: FnOnce()>(&self, f: F) -> Result<CmdHandle, io::Error> {
        let arg: Box<dyn FnOnce()> = Box::new(f);
        let arg = Box::new(arg);
        let arg_ptr = Box::into_raw(arg);

        let mut stack = vec![0u8; 1024 * 1000 * 8];
        let stack_ptr = unsafe { stack.as_mut_ptr().offset(1024 * 1000 * 8) };
        std::mem::forget(stack);

        extern "C" fn child(p: *mut libc::c_void) -> i32 {
            let p = p as *mut Box<dyn FnOnce()>;
            let inner = unsafe { Box::from_raw(p) };
            inner();
            0
        }

        let cmd = unsafe {
            libc::clone(
                child,
                stack_ptr as *mut libc::c_void,
                self.sys_proc_attr.clone_flags,
                arg_ptr as *mut libc::c_void,
            )
        };

        if cmd == -1 {
            Err(std::io::Error::last_os_error())
        } else {
            Ok(CmdHandle { handle: cmd })
        }
    }
}

fn run() {
    print!("Running: [");
    for cmd in env::args().skip(2) {
        print!("{} ", cmd);
    }
    println!("]");

    let cmd = Cmd::command(&env::args().nth(2).unwrap(), env::args().skip(2));

    let handle = cmd.run().unwrap();

    handle.wait().expect("Wait");
}
