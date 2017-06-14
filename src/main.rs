mod thunk;

use thunk::{Thunk, Args};

fn main() {
    let t = Thunk::newApp(Thunk::newNum(123.0), Args::new());
    println!("Hello, Tisp!");
}
