use rustyline::error::ReadlineError;
use rustyline::Editor;

pub struct Reader {
    reader: Editor<()>,
}

impl Reader {
    pub fn new() -> Reader {
        Reader {
            reader: Editor::<()>::new(),
        }
    }

    pub fn read_line(&mut self) -> Result<String, ReadlineError> {
        self.reader.readline("rust-dbg> ")
    }
}