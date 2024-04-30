use std::fs;

pub fn dump(text: &String, cli_user_path: &String) -> bool {
    let write_to = format!("{}\\dump\\response.txt", cli_user_path);
    let success = fs::write(write_to, text);
    
    match success{
        Ok(_) => return true,
        Err(_) => return false
    }
}