use rustyline::error::ReadlineError;
use rustyline::Editor;

pub fn sandbox() {
    let mut r1 = Editor::<()>::new();

    loop {
        let readline = r1.readline(">> ");
        match readline {
            Ok(line) => {
                r1.add_history_entry(line.as_str());
                println!("Line: {}", line);

                // forward to command dispatcher
            }
            Err(ReadlineError::Interrupted) => {
                println!("^C");
                // todo send signal to tracee
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("^D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}
