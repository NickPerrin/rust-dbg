use nix::errno::Errno::ESRCH;
use nix::sys::ptrace::cont;
use nix::sys::signal::{kill, SIGKILL};
use nix::unistd::Pid;

use crate::waitpid;

pub struct Tracer {
    pub pid: Pid,
    pub tracee: String,
}

impl Tracer {
    pub fn new(pid: Pid, tracee: String) -> Tracer {
        // @todo replace unwrap
        waitpid::wait_pid(pid).unwrap();
        Tracer {
            pid: pid,
            tracee: tracee,
        }
    }

    pub fn kill_tracee(&self) {
        println!("Killing tracee");
        if let Err(error) = kill(self.pid, SIGKILL) {
            match error {
                nix::Error::Sys(ESRCH) => println!("The tracee is not running"),
                _ => (),
            }
        }
    }

    pub fn continue_tracee(&self) {
        println!("Continuing tracee");
        // @todo replace expect
        cont(self.pid, None).expect("continue failed!");
        waitpid::wait_pid(self.pid).expect("waitpid failed after continue");
    }

    pub fn quit(&self) {
        println!("Goodbye!");
        match kill(self.pid, SIGKILL) {
            _ => (),
        }
    }
}
