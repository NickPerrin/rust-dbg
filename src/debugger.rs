use nix::errno::Errno::ESRCH;
use nix::sys::ptrace::cont;
use nix::sys::signal::{kill, SIGKILL};
use nix::unistd::Pid;
use std::io::Error;

use crate::parser::{Command, Parser};
use crate::reader::Reader;
use crate::waitpid;

const INTRO: &str = "\nWelcome to rust-dbg! \nuse help for more information\n";

#[derive(PartialEq)]
enum State {
    Started,
    Stopped,
    Exit,
}

pub struct Debugger {
    parser: Parser,
    reader: Reader,
    pid: Pid,
    state: State,
}

impl Debugger {
    pub fn new(pid: Pid, tracee: &str) -> Debugger {
        println!("{}", INTRO);
        println!("Debugging {} | pid {}", tracee, pid);

        // @todo replace unwrap
        waitpid::wait_pid(pid).unwrap();

        Debugger {
            parser: Parser::new(),
            reader: Reader::new(),
            pid: pid,
            state: State::Stopped,
        }
    }

    pub fn run(&mut self) -> Result<(), Error> {
        while self.state != State::Exit {
            self.handle_command();
        }
        Ok(())
    }

    fn handle_command(&mut self) {
        if let Ok(line) = self.reader.read_line() {
            match self.parser.parse_command(line) {
                Some(Command::Continue) => self.continue_tracee(),
                Some(Command::Kill) => self.kill_tracee(),
                Some(Command::Quit) => self.quit(),
                Some(Command::Run) => self.run_tracee(),
                _ => println!("unknown command"),
            }
        }
    }

    fn kill_tracee(&mut self) {
        match self.state {
            State::Started => {
                self.state = State::Stopped;
                if let Err(error) = kill(self.pid, SIGKILL) {
                    match error {
                        nix::Error::Sys(ESRCH) => println!("The tracee is not running"),
                        _ => (),
                    }
                }
            }
            _ => println!("The tracee is not running"),
        }
    }

    fn continue_tracee(&self) {
        match self.state {
            State::Started => {
                println!("Continuing");
                // @todo replace expect
                cont(self.pid, None).expect("continue failed!");
                waitpid::wait_pid(self.pid).expect("waitpid failed after continue command");
            }
            _ => println!("The tracee is not running"),
        }
    }

    fn run_tracee(&mut self) {
        match self.state {
            State::Stopped => {
                if let Err(_) = cont(self.pid, None) {
                    println!("Error running tracee");
                } else {
                    self.state = State::Started;
                }
                waitpid::wait_pid(self.pid).expect("waitpid failed after run command");
            }
            _ => println!("Restarting tracee"),
        }
    }

    fn quit(&mut self) {
        println!("Goodbye!");
        self.state = State::Exit;
        match kill(self.pid, SIGKILL) {
            _ => (),
        }
    }
}
