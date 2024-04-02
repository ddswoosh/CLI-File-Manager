use std::collections::HashMap;
use std::fmt::Display;
use std::path::PathBuf;
use std::env;

mod integrate;
mod controls;


fn main() { 

    
    let temp = &env::current_dir().expect("Path not found");
    let mut path: &dyn Display = &temp.display();

    let mut hm: &mut HashMap<String, String> = &mut controls::hm();

    integrate::read(&mut hm, &mut path);
    
}
