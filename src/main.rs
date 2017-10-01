#[macro_use]
extern crate scan_fmt;

use std::io::{self, Read};

mod dom;

fn main() {
    let mut source = String::new();
    let _ = io::stdin().read_to_string(&mut source);
    match dom::parse(source) {
        Ok(output) => println!("{}", output),
        Err(e) => eprintln!("{}", e.description),
    }
}
