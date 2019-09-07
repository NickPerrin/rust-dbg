use nix::sys::ptrace::cont;
use nix::sys::signal::{kill, SIGKILL};
use nix::unistd::Pid;

use crate::waitpid;

pub struct Tracer<'a> {
    pub pid: Pid,
    pub tracee: &'a str,
}

impl<'a> Tracer<'a> {
    pub fn new(pid: Pid, tracee: &str) -> Tracer {
        Tracer {
            pid: pid,
            tracee: tracee,
        }
    }

    pub fn kill_tracee(&self) {
        println!("Killing tracee");
        // @todo replace expect
        kill(self.pid, SIGKILL).expect("kill failed!");
    }

    pub fn continue_tracee(&self) {
        println!("Continuing tracee");
        // @todo replace expect
        cont(self.pid, None).expect("continue failed!");
        waitpid::wait_pid(self.pid).expect("waitpid failed after continue");
    }
}
