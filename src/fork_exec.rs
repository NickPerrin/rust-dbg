use nix::sys::ptrace::traceme;
use nix::sys::signal::{kill, SIGKILL};
use nix::unistd::{execv, fork, ForkResult, Pid};
use std::ffi::CString;
use std::process;

use crate::debugger;
use crate::waitpid;

pub fn fork_process(tracee: &str) {
    match fork() {
        Ok(ForkResult::Parent { child }) => {
            // @todo replace with proper error/result handling
            waitpid::wait_pid(child).unwrap();

            // @todo replace unwrap for better error handling
            debugger::initialize(child, &tracee).unwrap();

            // @todo remove when kill command is implemented
            kill_tracee(&child);
        }
        Ok(ForkResult::Child) => {
            match traceme() {
                Ok(_) => (),
                Err(_) => {
                    println!("tracee ptrace called failed");
                    process::exit(1);
                }
            };

            let tracee = match CString::new(tracee) {
                Ok(tracee_filename) => tracee_filename,
                _ => {
                    println!("Translation of tracee name failed");
                    process::exit(1);
                }
            };

            // @todo figure out how to pass arguements to the tracee
            match execv(&tracee, &[]) {
                Ok(_) => (),
                Err(_) => {
                    println!("Unable to start the child process");
                    process::exit(1);
                }
            };
        }
        _ => println!("Fork failed!"),
    };
}

// @todo possibly move into debugger module
fn kill_tracee(child: &Pid) {
    kill(*child, SIGKILL).expect("kill failed!");
}
