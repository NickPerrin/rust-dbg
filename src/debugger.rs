use nix::sys::ptrace::cont;
use nix::sys::signal::{kill, SIGKILL};
use nix::unistd::Pid;
use rustyline::error::ReadlineError;
use rustyline::Editor;

use crate::waitpid;

const INTRO: &str = "\nWelcome to rust-dbg! \nuse help for more information\n";

struct Reader {
    reader: Editor<()>,
}

impl Reader {
    fn new() -> Reader {
        Reader {
            reader: Editor::<()>::new(),
        }
    }

    fn read_line(&mut self) -> Result<String, ReadlineError> {
        self.reader.readline("rust-dbg> ")
    }
}

pub struct Tracer<'a> {
    pid: Pid,
    tracee: &'a str,
}

impl<'a> Tracer<'a> {
    fn new(pid: Pid, tracee: &str) -> Tracer {
        Tracer {
            pid: pid,
            tracee: tracee,
        }
    }

    fn kill_tracee(&self) {
        println!("Killing tracee");
        // @todo replace expect
        kill(self.pid, SIGKILL).expect("kill failed!");
    }

    fn continue_tracee(&self) {
        println!("Continuing tracee");
        // @todo replace expect
        cont(self.pid, None).expect("continue failed!");
        waitpid::wait_pid(self.pid).expect("waitpid failed after continue");
    }
}

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
