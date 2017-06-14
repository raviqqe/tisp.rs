mod thunk;

use thunk::{Thunk, Args};

fn main() {
    let t = Thunk::app(Thunk::num(123.0), Args::new());
    println!("Hello, Tisp!");
}
