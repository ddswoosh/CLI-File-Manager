use std::thread;
use std::env;
use std::fs;
use std::fmt::Display;
use std::path::PathBuf;
use std::time::Duration;
use std::collections::HashMap;

use crate::utils::cache;
use crate::routes::integrate;
use crate::controllers::controls;
use crate::middleware::response;

async fn open_cli() {
    let cli_user_path: String = controls::Environment::working_dir().unwrap().display().to_string();
    let exe_path: String = format!("{}\\cli\\app\\Rust-CLI.exe", &cli_user_path);

    std::process::Command::new("cmd")
                        .arg("/C")
                        .arg("start")
                        .arg(&exe_path)
                        .output();
}

pub async fn run() {
    let cli_user_path: String = controls::Environment::working_dir().unwrap().display().to_string();
    let mut global_user_path: String = String::new();
    let mut editors: HashMap<String, String> = HashMap::new();
    let mut j: usize = 0;

    for i in cli_user_path.chars() {
        if j < 8 {
            j += 1;
            continue
        }

        if i == '\\' {
            global_user_path = cli_user_path[..j].to_string();
        }
        j += 1;
    }

    let path_to_vscode: String = format!("{}\\AppData\\Local\\Programs\\Microsoft VS Code\\Code.exe", &global_user_path);
   
    editors.insert("code".to_string(), path_to_vscode);

    let mut cur_holding_path: [Option<PathBuf>; 1] = [None];
    let mut cur_holding_node: [Option<cache::Node>; 1] = [None];
    let mut extensions: HashMap<String, String> = controls::file_ext();
    let mut cur_command: String = String::new();

    let write_to: String = format!("{}\\dump\\command.txt", &cli_user_path);

    fs::write(&write_to, "");

    tokio::spawn(async {
        open_cli().await;
    });

    let mut shift: i8 = 0;
    let mut num_node: u8 = 0;
    let mut list: cache::List = cache::run();
    
    thread::sleep(Duration::from_millis(300));

    loop {
        if cur_command != fs::read_to_string(&write_to).unwrap() {
            let control_res: String = integrate::read(
                &mut extensions, &mut cur_holding_path, 
                &mut cur_holding_node, &mut editors, &mut cur_command, &mut list, 
                &mut num_node, &mut shift, &cli_user_path
            );

            let dump_res: bool = response::dump(&control_res, &cli_user_path);

            if control_res == "cache".to_string() {
                let dump_res: bool = response::dump(&"Please type a node number seen above to commence reversion".to_string(), &cli_user_path);
            }

            if dump_res == false {
                response::dump(&"Error writing to response to file, please restart the program".to_string(), &cli_user_path);
            }
        }
        thread::sleep(Duration::from_millis(50));
    }
}
