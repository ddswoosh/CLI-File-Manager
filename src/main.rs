use std::collections::HashMap;
use std::fmt::Display;
use std::path::PathBuf;
use std::time::Duration;
use std::env;
use std::thread;
use std::fs;

mod integrate;
mod controls;

fn main() { 
    let mut editors: HashMap<String, String> = HashMap::new();
    editors.insert("code".to_string(), "C:\\Users\\ddswoosh\\AppData\\Local\\Programs\\Microsoft VS Code\\Code.exe".to_string());

    let mut cur_holding: [Option<PathBuf>; 1] = [None];
    let mut hm: HashMap<String, String> = controls::file_ext();

    let mut cur_command: String = "".to_string();

    loop {

        if cur_command != fs::read_to_string("dump/command.txt").unwrap() {

            integrate::read(&mut hm, &mut cur_holding, &mut editors, &mut cur_command);
        } 

        thread::sleep(Duration::from_secs(1));
    }
}
