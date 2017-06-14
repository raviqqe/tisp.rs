extern crate docopt;
#[macro_use]
extern crate log;
#[macro_use]
extern crate nom;
#[macro_use]
extern crate serde_derive;

mod logger;
mod thunk;
mod parse;

use std::fs::File;
use std::io::Read;
use std::process::exit;
use thunk::Thunk;
use parse::parse;

#[derive(Debug, Deserialize)]
struct Args {
    arg_filename: String,
    flag_log_level: String,
}

fn get_args() -> Args {
    let usage = "
Tisp interpreter.

Usage:
    tisp [-l <log_level>] [<filename>]
    tisp (-h | --help)

Options:
    -l, --log-level <log_level>  Set log level [default: error].
    -h, --help  Show help.
";

    docopt::Docopt::new(usage)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit())
}

fn read_file(f: String) -> String {
    let mut s = String::new();

    match File::open(&f) {
        Ok(mut f) => {
            let n = f.read_to_string(&mut s).unwrap();
            assert_eq!(n, s.len());
            s
        }
        Err(s) => {
            error!("{}", s);
            exit(1)
        },
    }
}

fn main() {
    let args = get_args();

    logger::init(&args.flag_log_level);

    parse(read_file(args.arg_filename));

    Thunk::app(Thunk::num(123.0), thunk::Args::new());

    println!("Hello, Tisp!");
}
