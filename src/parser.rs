use maplit::hashmap;
use std::collections::HashMap;

pub enum Command {
    Continue,
    Kill,
    Quit,
    Run,
    Help,
    Step,
}

pub struct Parser {
    command_map: HashMap<String, Command>,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            command_map: hashmap! {
                "continue".to_string() => Command::Continue,
                "kill".to_string() => Command::Kill,
                "quit".to_string() => Command::Quit,
                "run".to_string() => Command::Run,
                "step".to_string() => Command::Step,
                "help".to_string() => Command::Help,
            },
        }
    }

    pub fn parse_command(&self, mut cmd: String) -> Option<&Command> {
        // for now, only supporting basic commands.
        // Will add support for sub-commands as necessary.
        // i.e "break <function name>"
        if cmd.chars().count() == 0 {
            return None;
        }
        self.expand(&mut cmd);
        self.command_map.get(&cmd)
    }

    fn expand(&self, cmd: &mut String) {
        for (key, _value) in self.command_map.iter() {
            if key.chars().next() == cmd.chars().next() {
                *cmd = key.clone();
                break;
            }
        }
        ()
    }
}
