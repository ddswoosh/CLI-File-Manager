use std::collections::HashMap;
use std::fmt::Display;
use std::path::PathBuf;
use std::env;

mod integrate;
mod controls;


fn main() { 

    let mut hm: &mut HashMap<String, String> = &mut controls::hm();

    integrate::read(&mut hm);
    
}
