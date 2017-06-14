extern crate docopt;
#[macro_use]
extern crate serde_derive;

mod thunk;

use thunk::Thunk;

#[derive(Debug, Deserialize)]
struct Args {
    arg_filename: String,
}

fn get_args() -> Args {
    let usage = "
Tisp interpreter.

Usage:
    tisp [<filename>]
    tisp (-h | --help)

Options:
    -h, --help  Show help.
";

    docopt::Docopt::new(usage)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit())
}

fn main() {
    let args = get_args();
    let t = Thunk::app(Thunk::num(123.0), thunk::Args::new());
    println!("Hello, Tisp!");
}
