use nix::unistd::Pid;
use std::io::Error;

use crate::parser::{Command, Parser};
use crate::reader::Reader;
use crate::tracer::Tracer;

const INTRO: &str = "\nWelcome to rust-dbg! \nuse help for more information\n";

#[derive(PartialEq)]
enum Status {
    Continue,
    Exit,
}

pub struct Debugger {
    parser: Parser,
    reader: Reader,
    status: Status,
    tracer: Tracer,
}

impl Debugger {
    pub fn new(pid: Pid, tracee: String) -> Debugger {
        println!("{}", INTRO);
        println!("Debugging {} | pid {}", tracee, pid);
        Debugger {
            parser: Parser::new(),
            reader: Reader::new(),
            status: Status::Continue,
            tracer: Tracer::new(pid, tracee),
        }
    }

    pub fn run(&mut self) -> Result<(), Error> {
        while self.status == Status::Continue {
            self.status = self.handle_command();
        }

        Ok(())
    }

    fn handle_command(&mut self) -> Status {
        if let Ok(line) = self.reader.read_line() {
            match self.parser.parse_command(line) {
                Some(Command::Continue) => self.tracer.continue_tracee(),
                Some(Command::Kill) => self.tracer.kill_tracee(),
                Some(Command::Quit) => {
                    self.tracer.quit();
                    return Status::Exit;
                }
                _ => println!("unknown command"),
            }
        }

        Status::Continue
    }
}
