use std::collections::HashMap;
use std::fmt::Display;
use std::path::PathBuf;
use std::time::Duration;
use std::env;
use std::thread;
use std::fs;
 
mod controllers;
mod routes;
mod middleware;

use crate::routes::integrate;
use crate::controllers::controls;
use crate::middleware::response;

fn main() { 
    let mut editors: HashMap<String, String> = HashMap::new();
    editors.insert("code".to_string(), "C:\\Users\\ddswoosh\\AppData\\Local\\Programs\\Microsoft VS Code\\Code.exe".to_string());

    let mut cur_holding: [Option<PathBuf>; 1] = [None];
    let mut hm: HashMap<String, String> = controls::file_ext();

    let mut cur_command: String = String::new();

    std::process::Command::new("cmd")
                            .arg("/C")
                            .arg("start")
                            .arg("C:\\Users\\ddswoosh\\rust\\cli\\app\\Rust-CLI.exe")
                            .output();

    loop {

        if cur_command != fs::read_to_string("dump\\command.txt").unwrap() {

            let control_res: Result<String, String> = integrate::read(&mut hm, &mut cur_holding, &mut editors, &mut cur_command);

            let dump_res: bool = response::dump(control_res);

            if dump_res == false {
                panic!("Fatal error writing to response file. File is either corrupted or missing.");
            }
            
            continue;
        }

        thread::sleep(Duration::from_secs(1));
    }
}
