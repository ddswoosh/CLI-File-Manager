use std::thread;
use std::env;
use std::fs;
use std::fmt::Display;
use std::path::PathBuf;
use std::time::Duration;
use std::collections::HashMap;


use crate::routes::integrate;
use crate::controllers::controls;
use crate::middleware::response;

async fn open_cli() {
    std::process::Command::new("cmd")
                        .arg("/C")
                        .arg("start")
                        .arg("C:\\Users\\ddswoosh\\rust\\cli\\app\\Rust-CLI.exe")
                        .output();
}

pub async fn run() {
    let mut editors: HashMap<String, String> = HashMap::new();
    editors.insert("code".to_string(), "C:\\Users\\ddswoosh\\AppData\\Local\\Programs\\Microsoft VS Code\\Code.exe".to_string());

    let mut cur_holding: [Option<PathBuf>; 1] = [None];
    let mut hm: HashMap<String, String> = controls::file_ext();

    let mut cur_command: String = String::new();
    fs::write("C:\\Users\\ddswoosh\\rust\\dump\\command.txt", "");

    tokio::spawn(async {
        open_cli().await;
    });

    thread::sleep(Duration::from_secs(3));

    loop {

        if cur_command != fs::read_to_string("C:\\Users\\ddswoosh\\rust\\dump\\command.txt").unwrap() {
            let control_res: Result<String, String> = integrate::read(&mut hm, &mut cur_holding, &mut editors, &mut cur_command);
            let dump_res: bool = response::dump(control_res.unwrap());

            if dump_res == false {
                response::dump("Error writing to resposne to file, please restart the program".to_string());
            }
        }
        thread::sleep(Duration::from_secs(1));
    }
}
