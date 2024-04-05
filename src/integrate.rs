use std::collections::HashMap;
use std::path::PathBuf;
use std::fmt::Display;
use std::fs;

use crate::controls;

pub fn read(hashmap: &mut HashMap<String, String>, cur_holding: &mut [Option<PathBuf>; 1]) -> Result<String, String> {
    let path: PathBuf = controls::working_dir().expect("Non-fatal error");

    let cin = fs::read_to_string("dump/command.txt").map_err(|e| e.to_string())?;
    let vec: Vec<&str> = cin.split_whitespace().collect();
    let command: &str = &vec[0];
    // let name: &str = &vec[1];
    // let ext: &str = &vec[2];

    match command {
        // "nd" => controls::Dir::new_directory(name.to_string(), path),
        // "dd" => controls::Dir::delete_directory(name.to_string(), path),
        // "odd" => controls::Dir::override_delete(name.to_string(), path),
        // "nf" => controls::Nfil::new_file(name.to_string(), ext.to_string(), hashmap),
        // "df" => controls::Dfil::delete_file(name.to_string(), path),
        // "mov" => controls::Mov::mov(name.to_string(), ext.to_string()),
        // "grab" => controls::FileArray::grab(name.into(), cur_holding),
        "drop" => controls::FileArray::drop(cur_holding),
        &_ => todo!()
    };

    return Ok(command.to_string());
}

