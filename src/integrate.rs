use::std::io;
use::std::fs;

mod controls;

pub fn read() => Result<(), String> {
    let command = fs::read_to_string("dump/response.txt").parse()?;
    match command {
        String::from("ndir")=> new(new_directory),
        Err(err) => String::from("Not able to parse command.")
    }
        
}

