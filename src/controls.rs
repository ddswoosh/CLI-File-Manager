use std::io;
use std::fs;
use std::env;
use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;
use std::fmt::Display;
use std::process::Command;
use std::string::String;

use crate::integrate;

enum Action {

    Grab,
    Drop,
    Move,
    Newfile,
    Deletefile,
    NewDirectory,
    DeleteDirectory,
    Open,
    Search,
}

pub struct Environment;

pub struct FileArray<PathBuf> {
    cur_holding: PathBuf,
    name: PathBuf,  
}

pub struct Mov<T> {
    init_path: T,
    end_path: T,
}

pub struct Fil<String> {
    file_name: String,
    file_type: String,
}


pub struct Dir<String> {
    directory_name: String,
}

pub struct Open<T> {
    editor_name: T,
    file_name: T,
}
pub struct Search<String> {
    root: String,
    key: Option<String>,
}

pub fn file_ext() -> HashMap<String, String> {
    let mut file_map: HashMap<String, String> = HashMap::new();

    file_map.insert("python".to_string(), ".py".to_string());
    file_map.insert("java".to_string(), ".java".to_string());
    file_map.insert("javascript".to_string(), ".js".to_string());
    file_map.insert("c++".to_string(), ".cpp".to_string());
    file_map.insert("c".to_string(), ".c".to_string());
    file_map.insert("rust".to_string(), ".rs".to_string());
    file_map.insert("text".to_string(), ".txt".to_string());
    file_map.insert("pdf".to_string(), ".pdf".to_string());
    file_map.insert("executable".to_string(), ".exe".to_string());
    file_map.insert("json".to_string(), ".json".to_string());

    return file_map;
}

pub fn file_ext_insert(full: String, ext: String, hashmap: &mut HashMap<String, String>) -> &mut HashMap<String, String> {
    hashmap.insert(full, ext);

    return hashmap;
}

impl FileArray<PathBuf> {
    pub fn drop(cur_holding: &mut [Option<PathBuf>; 1]) -> Result<String, String> {
        cur_holding[0] = None;  
        
        return Ok("Success".to_string());
    }

    pub fn grab(name: PathBuf, cur_holding: &mut [Option<PathBuf>; 1]) -> Result<String, String>{
        let mut cur_path: PathBuf = Environment::working_dir().unwrap();
        cur_path.push(name);

        cur_holding[0] = Some(cur_path);

        match cur_path {
            _ => return Ok("Success".to_string()),
            _ => return Err("Could not grab.".to_string())
        }
    }
}

impl Mov<String> {
    pub fn mov(from: String, to: String) -> Result<String, String> {
        let mut cur_path: PathBuf = Environment::working_dir().expect("Err");
        cur_path.push(from);
        
        let attempt: Result<(), std::io::Error> = fs::rename(cur_path, to);

        match attempt {
            Ok(()) => Ok("Success".to_string()),
            Err(_) =>Err("Destination is not valid, please try again.".to_string())
        }
    }
}

impl Fil<String> {
    pub fn new_file(file_name: String, file_ext: String, hm: &mut HashMap<String, String>) -> Result<String, String> {

        if hm.get(&file_ext).is_some() {
            let ext: &String = hm.get(&file_ext).unwrap();
            let name: String = file_name + &ext;

            fs::File::create(name);
            return Ok("Success".to_string());

        } else {
            return Err("Your chosen file type does not yet exist. To create this extension, please type 
            'add, full extension name, extension' (i.e. add python .py).".to_string());
        } 
    } 

    pub fn delete_file(file_name: String, path: PathBuf) -> Result<String, String> {
        let success = fs::remove_file(file_name);
        
        match success {
            Ok(()) => return Ok("Success".to_string()),
            Err(_) => Err("Check the file exists or the your spelling is correct.".to_string())
        }
    }
}

impl Dir<String> {
    pub fn new_directory(dir_name: String, path: PathBuf) -> Result<String, String> {
        let success: Result<(), std::io::Error> = fs::create_dir(dir_name);
        
        match success {
            Ok(()) => return Ok("Success".to_string()),
            Err(_) => return Err("Path already exists".to_string())
        }
    } 

    pub fn delete_directory(dir_name: String, path: PathBuf) -> Result<String, String> {
        let del: Result<(), std::io::Error> = fs::remove_dir(dir_name);

        match del {
            Ok(()) => return Ok("Success".to_string()),
            Err(_) => return Err("The directory is not empty, please remove all contents or type 'oddir {directory name}' if you want to force
            delete all child content. Type cancel to exit this function".to_string()),
        }

    }
    pub fn override_delete(dir_name: String, path: PathBuf) -> Result<String, String> {
        let del: Result<(), std::io::Error> = fs::remove_dir_all(dir_name);
        
        match del {
            Ok(()) => return Ok("Success".to_string()),
            Err(_) => return Err("Non-fatal error".to_string()),
        }
    }
}

impl Open<String> {
    pub fn open(editor_name: String, file_name: String, editors: &mut HashMap<String, String>) -> Result<String, String> {
        let mut cur_path: PathBuf = Environment::working_dir().expect("Err");
        cur_path.push(&file_name);

        if editors.get(&editor_name).is_some() {
            let status = Command::new(editors.get(&editor_name).unwrap()).arg(cur_path).status();

            match status {
                Ok(status) => return Ok("Success".to_string()),
                Err(_) => return Err("Could not open editor".to_string())
            };     
        }

        return Err("Editor not found, if you want to add your own editor, type help and follow the add editor instructions".to_string());
    }

    pub fn add_editor(editor_name: String, path: String, editors: &mut HashMap<String, String>) -> Result<String, String> {
        let success: Option<String> = editors.insert(editor_name, path);

        match success { 
            Some(_) => return Ok("Success".to_string()),
            None => return Err("Error adding editor to list".to_string())
        }
    }
}

impl Search<String> {
    pub fn list_dir(root: String) -> Result<String, String> {
        let mut children: Vec<PathBuf> = vec![];

        let mut cur: PathBuf = Environment::working_dir().unwrap();

        cur.push(root);
        
        if cur.is_dir() {
            for child in fs::read_dir(cur).unwrap() {
                let child: fs::DirEntry = child.unwrap();
                let mut path: PathBuf = child.path();
    
                if path.is_dir() {
                    path.push("-> Directory");
                    children.push(path);
                } else {
                    path.push("-> File");
                    children.push(path);
                }
            }
        } else {
            return Err("This file name is not of type: Directory".to_string());
        }

        if children.len() == 0 {
            return Err("Directory has no children".to_string());
        } else {
            let res: &mut PathBuf = &mut Default::default();
            
            for i in 0..children.len() {
                res.push(&children[i]);
            }
            return Ok(res.display().to_string());
        }
    }

    pub fn find_file(find: &String) -> Result<String, String> {
        match Path::new(&find).exists() {
            true => Ok("File found".to_string()),
            false => Err("File does not exist or permissions are restricted".to_string())
        }
    }
}

impl Environment {
    pub fn working_dir() -> Result<PathBuf, std::io::Error> {
        let cur: Result<PathBuf, std::io::Error> = env::current_dir();  
        
        return Ok(cur?);  
    }

    pub fn change_dir(change: String) -> Result<String, String> {
        let exists: Result<String, String> = Search::find_file(&change);

        match exists {
            Ok(_) => {
                let mut cur: PathBuf = Self::working_dir().unwrap();
                cur.push(change);

                let success: Result<(), std::io::Error> = env::set_current_dir(cur);

                match success {
                    Ok(()) => return Ok("Success".to_string()),
                    Err(_) => return Err("Error changing directory".to_string())
                }
            }

            Err(_) => return Err("Change to directory does not exsist within the scope of your current directory".to_string())
        }
    }
}

pub fn dump(text: Result<String, String>) -> bool {
    match text {
        Ok(_) => {
            fs::write("dump\\response.txt", text.unwrap());
            return true; 
        },

        Err(_) => return false
    }
}