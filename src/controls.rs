use std::io;
use std::fs;
use std::env;
use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;
use std::fmt::Display;
use std::process::Command;

use crate::integrate;

// enum Action {

//     Start,
//     Grab,
//     Move,
//     Newfile,
//     Deletefile,
//     NewDirectory,
//     DeleteDirectory,
//     Drop,
//     Open,
//     Search,
// }

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

pub fn hm() -> HashMap<String, String> {
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

pub fn hm_insert(full: String, ext: String, hashmap: &mut HashMap<String, String>) {
    hashmap.insert(full, ext);
    
    println!("If your entered file type is not supported in Windows, the file may not open as intended.");
}

impl FileArray<PathBuf> {
    pub fn drop(cur_holding: &mut [Option<PathBuf>; 1]) {
        cur_holding[0] = None;   
        
        println!("{:?}", cur_holding);
    }

    pub fn grab(name: PathBuf, cur_holding: &mut [Option<PathBuf>; 1]) {
        let mut cur_path: PathBuf = Environment::working_dir().expect("Err");
        cur_path.push(name);

        cur_holding[0] = Some(cur_path); 
    }
}

impl Mov<String> {
    pub fn mov(from: String, to: String) -> Result<(), String> {
        let mut cur_path: PathBuf = Environment::working_dir().expect("Err");
        cur_path.push(from);
        
        let attempt: Result<(), std::io::Error> = fs::rename(cur_path, to);

        match attempt {
            Ok(()) => Ok(()),
            Err(err) =>Err("Destination is not valid, please try again.".to_string())
        }
    }
}


impl Fil<String> {
    pub fn new_file(file_name: String, file_ext: String, hm: &mut HashMap<String, String>, path: PathBuf) -> Result<(), String> {

        if hm.get(&file_ext).is_some() {
            let ext: &String = hm.get(&file_ext).unwrap();
            let name: String = file_name + &ext;

            fs::File::create(name);
            return Ok(());

        } else {
            return Err("Your chosen file type does not yet exist. To create this extension, please type 
            'add, full extension name, extension' (i.e. add python .py).".to_string());
        } 
    } 

    pub fn delete_file(file_name: String, path: PathBuf) -> Result<(), String> {
        let success = fs::remove_file(file_name);
        
        match success {
            Ok(()) => return Ok(()),
            Err(err) => Err("Check the file exists or the your spelling is correct.".to_string())
        }
    }
}

impl Dir<String> {
    pub fn new_directory(dir_name: String, path: PathBuf) -> Result<(), String> {
        let success: Result<(), std::io::Error> = fs::create_dir(dir_name);
        
        match success {
            Ok(()) => return Ok(()),
            Err(err) => return Err("Path already exists".to_string())
        }
    } 

    pub fn delete_directory(dir_name: String, path: PathBuf) -> Result<(), String> {
        let del: Result<(), std::io::Error> = fs::remove_dir(dir_name);

        match del {
            Ok(()) => return Ok(()),
            Err(err) => return Err("The directory is not empty, please remove all contents or type 'oddir {directory name}' if you want to force
            delete all child content. Type cancel to exit this function.".to_string()),
        }

    }
    pub fn override_delete(dir_name: String, path: PathBuf) -> Result<(), String> {
        let del: Result<(), std::io::Error> = fs::remove_dir_all(dir_name);
        
        match del {
            Ok(()) => return Ok(()),
            Err(err) => return Err("Non-fatal error".to_string()),
        }
    }
}

impl Open<String> {
    pub fn open(editor_name: String, file_name: String, editors: &mut HashMap<String, String>) -> Result<(), String> {
        let mut cur_path: PathBuf = Environment::working_dir().expect("Err");
        cur_path.push(&file_name);


        if editors.get(&editor_name).is_some() {
            let status = Command::new(editors.get(&editor_name).unwrap()).arg(cur_path).status();

            match status {
                Ok(status) => return Ok(()),
                Err(err) => return Err("Could not open editor.".to_string())
            };     
        }

        return Err("Editor not found. If you want to add your own editor, type help and follow the add editor instructions.".to_string());
    }

    pub fn add_editor(editor_name: String, path: String, editors: &mut HashMap<String, String>) -> Result<(), String> {
        let success: Option<String> = editors.insert(editor_name, path);

        match success { 
            Some(_) => return Ok(()),
            None => return Err("Error adding editor to list.".to_string())
        }
    }
}

impl Search<String> {
    pub fn list_dir(root: String) -> Result<(), String> {
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
            return Err("This file name is not of type: Directory.".to_string());
        }

        println!("{:?}", children);

        if children.len() == 0 {
            return Err("Directory has no children.".to_string());
        } else {
            return Ok(());
        }
    }

    pub fn find_file(find: &String) -> Result<(), String> {
        match Path::new(&find).exists() {
            true => Ok(()),
            false => Err("File does not exist or permissions are restricted.".to_string())
        }
    }
}

impl Environment {
    pub fn working_dir() -> Result<PathBuf, std::io::Error> {
        let cur: Result<PathBuf, std::io::Error> = env::current_dir();
        
        return Ok(cur?);  
    }

    pub fn change_dir(change: String) -> Result<(), String> {
        let exists: Result<(), String> = Search::find_file(&change);

        match exists {
            Ok(()) => {
                let mut cur: PathBuf = Self::working_dir().unwrap();
                cur.push(change);

                println!("{:?}", cur.display());

                let success: Result<(), std::io::Error> = env::set_current_dir(cur);

                match success {
                    Ok(()) => return Ok(()),
                    Err(err) => return Err("Error changing directory.".to_string())
                }
            }

            Err(err) => return Err("Change to directory does not exsist within the scope of your current directory.".to_string())
        }
    }
}