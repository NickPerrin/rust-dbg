use nix::unistd::Pid;

use crate::reader::Reader;
use crate::tracer::Tracer;
use crate::waitpid;

const INTRO: &str = "\nWelcome to rust-dbg! \nuse help for more information\n";

pub fn initialize(pid: Pid, tracee: &str) -> Result<Tracer, &'static str> {
    let tracer = Tracer::new(pid, &tracee);
    println!("{}", INTRO);
    println!("Debugging {} | pid {}", tracer.tracee, tracer.pid);

    // @todo replace unwrap
    waitpid::wait_pid(tracer.pid).unwrap();
    Ok(tracer)
}

pub fn run(tracer: Tracer) -> Result<(), &'static str> {
    let mut reader = Reader::new();

    if let Ok(line) = reader.read_line() {
        match line.as_ref() {
            "continue" => tracer.continue_tracee(),
            "kill" => tracer.kill_tracee(),
            _ => println!("Unknown command!"),
        }
    }

    Ok(())
}
