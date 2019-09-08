use nix::unistd::Pid;

use crate::parser::{Command, Parser};
use crate::reader::Reader;
use crate::tracer::Tracer;
use crate::waitpid;

const INTRO: &str = "\nWelcome to rust-dbg! \nuse help for more information\n";

#[derive(PartialEq)]
enum Status {
    Continue,
    Exit,
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
    let parser = Parser::new();
    let mut status = Status::Continue;

    while status == Status::Continue {
        status = handle_command(&parser, &mut reader, &tracer);
    }

    Ok(())
}

fn handle_command(parser: &Parser, reader: &mut Reader, tracer: &Tracer) -> Status {
    if let Ok(line) = reader.read_line() {
        match parser.parse_command(line) {
            Some(Command::Continue) => tracer.continue_tracee(),
            Some(Command::Kill) => tracer.kill_tracee(),
            Some(Command::Quit) => {
                tracer.quit();
                return Status::Exit;
            }
            _ => println!("unknown command"),
        }
    }

    Status::Continue
}
