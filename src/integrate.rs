use std::fs;
use crate::controls;



pub fn read() -> Result<String, String> {
    let cin = fs::read_to_string("dump/command.txt").map_err(|e| e.to_string())?;
    let vec: Vec<&str> = cin.split_whitespace().collect();
    let command: &str = &vec[0];
    let name: &str = &vec[1];

    println!("{} {}", command, name);

    match command {
        "ndir" => controls::Ndir::new_directory(name.to_string()),
        "ddir" => controls::Ddir::delete_directory(name.to_string()),
        "oddir" => controls::Ddir::override_delete(name.to_string()),
        &_ => todo!()
    }?;


    return Ok(command.to_string());
        
}

