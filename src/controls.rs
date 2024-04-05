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
    pub fn open(editor_name: String, file_name: String) -> Result<(), String> {
        // need to manually add path to each editor later, vs code works for right now
        
        let mut editors: Vec<String> = vec!["code".to_string(), "vim".to_string(), "notepad".to_string(), "sublime".to_string()];

        let mut cur_path: PathBuf = Environment::working_dir().expect("Err");
        cur_path.push(&file_name);

        for i in editors.iter() {
             if editor_name == *i {
                let status = Command::new("C:\\Users\\ddswoosh\\AppData\\Local\\Programs\\Microsoft VS Code\\Code.exe").arg(cur_path).status();

                match status {
                    Ok(status) => return Ok(()),
                    Err(err) => return Err("Could not open editor.".to_string())
                };
             }
        }

        return Err("Editor not found in vector. If you want to add your own editor, type help and follow the add editor instructions.".to_string());
    }

    // fn add_editor(editor_name: String, path: PathBuf, editors: mut Vec<String>) {


    // }
}

impl Search<String> {
    // fn list_dir(root: String>) -> Option<Vec<String>> {
    //     let mut q: Vec<String> = vec![root];
    //     let mut res: Vec<Vec<String>> = vec![];
        
    //     while q.len() > 0 {
    //         let mut level: Vec<String> = vec![];

    //         for children in fs::read_dir(root)? {
    //             let cur = children?;
    //             match cur {
    //                 Err(err) => return Some(res);
    //             }

    //             let cd: String = cur.path();

    //             if cd.is_dir() {
    //                 q.push(&cd);
    //             } else {
    //                 level.push(cd);
    //             }
    //         }

    //         res.push(level)
    //     }
        
    //     if res.len() > 0 {
    //         return Some(res)
    //     }

    //     return None;
    // }

    pub fn find_file(find: String) -> Result<(), String> {
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
        let exists: Result<(), String> = Search::find_file(change);

        match exists {
            Err(err) => return Err("Change to directory does not exsist within the scope of your current directory.".to_string())
        }
        
        let cur: Result<PathBuf, std::io::Error> = Self::working_dir();
        let new_path: PathBuf = cur.push(&change);

        let success: Result<(), std::io::Error> = env::set_current_dir(new_path);

        match success {
            Ok(()) => return Ok(()),
            Err(err) => return Err("Error changing directory.".to_string())
        }

    }
}