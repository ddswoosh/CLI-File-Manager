use std::collections::HashMap;
use std::path::PathBuf;
use std::fmt::Display;
use std::fs;

use crate::controllers::controls;

pub fn read(hashmap: &mut HashMap<String, String>, cur_holding: &mut [Option<PathBuf>; 1], editors: &mut HashMap<String, String>, cur_command: &mut String) -> Result<String, String> {
    let path: PathBuf = controls::Environment::working_dir().expect("Non-fatal error");

    let cin: String = fs::read_to_string("dump/command.txt").map_err(|e| e.to_string())?;
    
    *cur_command = cin.clone();

    let vec: Vec<&str> = cin.split_whitespace().collect();

    
    let mut command: &str = "";
    let mut name: &str = "";
    let mut ext: &str = "";

    if vec.len() == 3 {
        command = &vec[0];
        name = &vec[1];
        ext = &vec[2];

        match command {
            "nf" => return controls::Fil::new_file(name.to_string(), ext.to_string(), hashmap),
            "open" => return controls::Open::open(ext.to_string(), name.to_string(), editors),
            "add" => return controls::Open::add_editor(name.to_string(), ext.to_string(), editors),
            "open" => return controls::Open::open(ext.to_string(), name.to_string(), editors),
            "mov" => {
                if name == "holding".to_string(){
                    return controls::Mov::mov(cur_holding[0].clone().unwrap().display().to_string(), ext.to_string())
                    } 
                    return controls::Mov::mov(name.to_string(), ext.to_string())
            },
            _ => return Ok("Command not accepted, please type -help to see the list of all commands.".to_string())
    }
    

    } else if vec.len() == 2 {
        command = &vec[0];
        name = &vec[1];

        match command {
            "nd" => return controls::Dir::new_directory(name.to_string(), path),
            "dd" => return controls::Dir::delete_directory(name.to_string(), path),
            "odd" => return controls::Dir::override_delete(name.to_string(), path),
            "df" => return controls::Fil::delete_file(name.to_string(), path),
            "grab" => return controls::FileArray::grab(name.into(), cur_holding),
            "cd" =>  {
                // if name == "back" {

                // }
                return controls::Environment::change_dir(name.to_string())
                }

            _ => return Ok("Command not accepted, please type -help to see the list of all commands.".to_string())
        }

    } else if vec.len() == 1 {
        command = &vec[0];

        // if command == "wd" {
        //     return controls::Environment::working_dir().display().to_string();
        // }

        match command {
            "drop" => return controls::FileArray::drop(cur_holding),
            "show" => return controls::FileArray::show(cur_holding),
            "list" => return controls::Search::list_dir(),
            _ => return Ok("Command not accepted, please type -help to see the list of all commands.".to_string())
        }

    } else {
        return Ok("Command not accepted, please type -help to see the list of all commands.".to_string());
    }
}

