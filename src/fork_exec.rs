use nix::sys::signal::*;
use nix::unistd::{fork, ForkResult, sleep};

pub fn fork_process() {
    match fork() {
        Ok(ForkResult::Parent { child }) => {
            println!("This is the parent. The child pid is {}", child);
            sleep(2);
            kill(child, SIGKILL).expect("kill failed!");
        }
        Ok(ForkResult::Child) => {
            loop {
                println!("This is the child");
            }

            // @todo get the name of the tracee
            // @todo call ptrace TRACE_ME
            // @todo call execv with tracee name and cmdline args
        }
        _ => println!("Fork failed!")
    };
}