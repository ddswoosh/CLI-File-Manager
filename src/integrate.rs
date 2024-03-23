use std::fs::OpenOptions;
use std::io::prelude::*;

pub fn pipe() {
    let mut file = OpenOptions::new()
        .read(true)
        .open(r"\\.\pipe\testpipe")
        .unwrap();


    let mut buffer = String::new();


    file.read_to_string(&mut buffer).unwrap();


    println!("Received data: {}", buffer);
}
