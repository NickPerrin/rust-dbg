use nix::sys::ptrace::traceme;
use nix::sys::signal::{kill, SIGKILL};
use nix::unistd::{execv, fork, sleep, ForkResult};
use std::ffi::CString;
use std::process;

pub fn fork_process(tracee: &str) {
    match fork() {
        Ok(ForkResult::Parent { child }) => {
            println!("This is the parent. The child pid is {}", child);
            sleep(2);
            kill(child, SIGKILL).expect("kill failed!");
        }
        Ok(ForkResult::Child) => {
            println!("This is the child process");
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
