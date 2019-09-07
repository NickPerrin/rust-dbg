use nix::sys::ptrace::traceme;
use nix::sys::signal::{kill, SIGKILL};
use nix::unistd::{execv, fork, ForkResult};
use std::ffi::CString;
use std::process;

use crate::debugger;

pub fn fork_process(tracee: &str) {
    match fork() {
        // This is the parent process(tracer)
        Ok(ForkResult::Parent { child }) => {
            // @todo replace with proper error/result handling
            //waitpid::wait_pid(child).unwrap();

            // @todo replace unwrap for better error handling
            if let Ok(tracer) = debugger::initialize(child, &tracee) {
                debugger::run(tracer).unwrap();
                return;
            }

            kill(child, SIGKILL).expect("kill failed!");
        }
        // This is the child process(tracee)
        Ok(ForkResult::Child) => {
            match traceme() {
                Ok(_) => (),
                Err(_) => {
                    println!("tracee ptrace call failed");
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

            // @todo figure out how to pass arguments to the tracee
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
