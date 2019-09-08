use maplit::hashmap;
use std::collections::HashMap;

pub enum Command {
    Continue,
    Kill,
    Quit,
    Run,
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
            },
        }
    }

    pub fn parse_command(&self, cmd: String) -> Option<&Command> {
        // for now, only supporting basic commands.
        // Will add support for sub-commands as necessary.
        // i.e "break <function name>"

        self.command_map.get(&cmd.to_lowercase())
    }
}
