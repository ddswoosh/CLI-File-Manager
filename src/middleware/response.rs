use std::fs;

pub fn dump(text: Result<String, String>) -> bool {
    match text {
        Ok(_) => {
            fs::write("C:\\Users\\ddswoosh\\rust\\dump\\response.txt", text.unwrap());
            return true; 
        },
        Err(_) => return false
    }
}