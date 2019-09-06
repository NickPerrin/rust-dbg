use clap::{App, Arg};
use std::process;

mod fork_exec;
mod rsline;

fn main() {
    let matches = App::new("rust-dbg")
        .version(clap::crate_version!())
        .about("Basic debugger(just use gdb instead)")
        .author(clap::crate_authors!())
        .arg(
            Arg::with_name("target")
                .short("t")
                .long("target")
                .value_name("Inferior")
                .help("The program to be debugged")
                .required(true)
                .takes_value(true)
                .index(1),
        )
        .get_matches();

    let target = matches.value_of("target").unwrap_or_else(|| {
        println!("No value provided for target");
        process::exit(1);
    });

    println!("target: {}", target);

    //rsline::sandbox();
    fork_exec::fork_process(&target);
}
