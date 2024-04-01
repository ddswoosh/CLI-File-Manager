use std::collections::HashMap;
use std::fs;
use crate::controls;

pub fn read(hashmap: &mut HashMap<String, String>) -> Result<String, String> {

    let cin = fs::read_to_string("dump/command.txt").map_err(|e| e.to_string())?;
    let vec: Vec<&str> = cin.split_whitespace().collect();
    let command: &str = &vec[0];
    let name: &str = &vec[1];
    // let ext: &str = &vec[2];


    println!("{} {}", command, name);

    match command {
        "nd" => controls::Ndir::new_directory(name.to_string()),
        "dd" => controls::Ddir::delete_directory(name.to_string()),
        "odd" => controls::Ddir::override_delete(name.to_string()),
        // "nf" => controls::Nfil::new_file(name.to_string(), ext.to_string(), hashmap),
        "df" => controls::Dfil::delete_file(name.to_string()),
        &_ => todo!()
    }?;

    return Ok(command.to_string());
}

