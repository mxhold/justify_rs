extern crate justify;

use justify::justify;

use std::io::{self, Read};


pub fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let output = justify(&buffer, 20);
    print!("{}", output);
}
