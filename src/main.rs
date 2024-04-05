use std::collections::HashMap;
use std::fmt::Display;
use std::path::PathBuf;
use std::env;

mod integrate;
mod controls;


fn main() { 

    let mut cur_holding: [Option<PathBuf>; 1] = [None];
    let mut hm: HashMap<String, String> = controls::hm();

    integrate::read(&mut hm, &mut cur_holding);
}
