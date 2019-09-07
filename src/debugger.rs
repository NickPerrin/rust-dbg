use nix::unistd::Pid;
use rustyline::error::ReadlineError;
use rustyline::Editor;

const INTRO: &str = "\nWelcome to rust-dbg! \nuse --help for more information\n";

pub struct Tracer<'a> {
    reader: Editor<()>,
    pid: Pid,
    tracee: &'a str,
}

impl<'a> Tracer<'a> {
    fn new(pid: Pid, tracee: &str) -> Tracer {
        Tracer {
            reader: Editor::<()>::new(),
            pid: pid,
            tracee: tracee,
        }
    }

    fn read_line(&mut self) -> Result<String, ReadlineError> {
        self.reader.readline("rust-dbg> ")
    }
}

pub fn initialize(pid: Pid, tracee: &str) -> Result<(), &str> {
    let mut tracer = Tracer::new(pid, &tracee);
    println!("{}", INTRO);
    if let Ok(line) = tracer.read_line() {
        println!("{}", line);
    } else {
        println!("readline failed");
    }

    Ok(())
}
