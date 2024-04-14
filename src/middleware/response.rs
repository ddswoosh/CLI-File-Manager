use std::fs;

pub fn dump(text: String) -> bool {
    let success = fs::write("C:\\Users\\ddswoosh\\rust\\dump\\response.txt", text);
    
    match success{
        Ok(()) => return true,
        Err(err) => return false
    }
}