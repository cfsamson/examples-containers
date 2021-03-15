use std::ffi::{CString};
use std::{io, ptr};
use libc::{self, c_char};


pub struct CmdHandle {
    handle: i32,
    stack_low_ptr: *mut u8,
}

impl CmdHandle {
    pub fn wait(self) -> Result<(), io::Error> {
        let wait_res = unsafe { libc::waitpid(self.handle, ptr::null_mut(), libc::__WALL) };
        let stack = unsafe {Vec::from_raw_parts(self.stack_low_ptr, STACK_SIZE, STACK_SIZE)};
        drop(stack);

        if wait_res == -1 {
            Err(std::io::Error::last_os_error())
        } else {
            Ok(())
        }
    }
}

pub struct Cmd {
    cmd: CString,
    pub sys_proc_attr: SysProcAttr,
    args: Vec<CString>,
}

#[derive(Default)]
pub struct SysProcAttr {
    pub clone_flags: i32,
}

const STACK_SIZE: usize =  1024 * 1000 * 8;

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

    /// Runs the provided closure in the new process before executing the
    /// command
    pub fn run_with_proc(self, proc: impl FnOnce()) -> Result<CmdHandle, io::Error> {
        self.run_proc(|| {
            proc();
            let mut args: Vec<*const c_char> = self.args.iter().map(|a| a.as_ptr()).collect();
            args.push(ptr::null());

            let res = unsafe { libc::execvp(self.cmd.as_ptr(), args.as_ptr()) };

            if res == -1 {
                panic!("execl err: {}", io::Error::last_os_error());
            }
        })
    }

    pub fn run(self) -> Result<CmdHandle, io::Error> {
        self.run_with_proc(|| {})
    }

    pub fn run_proc<F: FnOnce()>(&self, f: F) -> Result<CmdHandle, io::Error> {
        let arg: Box<dyn FnOnce()> = Box::new(f);
        let arg = Box::new(arg);
        let arg_ptr = Box::into_raw(arg);

        let mut stack = vec![0u8;STACK_SIZE];
        let stack_low_ptr = stack.as_mut_ptr();
        let stack_ptr = unsafe { stack.as_mut_ptr().offset(STACK_SIZE as isize) };
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
            Ok(CmdHandle { handle: cmd, stack_low_ptr, })
        }
    }
}