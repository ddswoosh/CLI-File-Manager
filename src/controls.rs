use std::io;
use std::fs;
use std::env;
use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;
use std::fmt::Display;

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

pub struct Grab<T> {
    grab_name: T,
}

pub struct Mov<T> {
    init_path: T,
    end_path: T,
}

pub struct Nfil<T> {
    file_name: T,
    file_type: T,
}

pub struct Dfil<T> {
    file_name: T,
}

pub struct Ndir<T> {
    directory_name: T,
}

pub struct Ddir<T> {
    directory_name: T,
}

pub struct Drop<T> {
    held: T,
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

// impl Grab<String> {
//     fn grab(name: String) {
//         cur_holding[0] = name;    
//     }
// }

// impl Move<String> {
//     fn mov(from: String, to: String, name: String) -> Result<(), E> {
//         let cur: String = search(from);

//         // Search will check if the from file is valid.
//         // If there was an error, it would be with the destination.

//         let attempt: Result<(), E> = fs::rename(cur, to);

//         match attempt {
//             Ok(()) => Ok("File successfully created "),
//             Err(err) => Err("Destination is not valid, please try again. ")
//         }

//     }
// }


impl Nfil<String> {
    pub fn new_file(file_name: String, file_ext: String, hm: &mut HashMap<String, String>) -> Result<(), String> {

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
}

impl Dfil<String> {
    pub fn delete_file(file_name: String) -> Result<(), String> {
        let success = fs::remove_file(file_name);
        
        match success {
            Ok(()) => return Ok(()),
            Err(err) => Err("Check the file exists or the your spelling is correct.".to_string())
        }
    }
}

impl Ndir<String> {
    pub fn new_directory(dir_name: String) -> Result<(), String> {
        let success: Result<(), std::io::Error> = fs::create_dir(dir_name);
        
        match success {
            Ok(()) => return Ok(()),
            Err(err) => return Err("Path already exists".to_string())
        }
    } 
}

impl Ddir<String> {
    pub fn delete_directory(dir_name: String) -> Result<(), String> {
        let del: Result<(), std::io::Error> = fs::remove_dir(dir_name);

        match del {
            Ok(()) => return Ok(()),
            Err(err) => return Err("The directory is not empty, please remove all contents or type 'oddir {directory name}' if you want to force
            delete all child content. Type cancel to exit this function.".to_string()),
        }

    }
    pub fn override_delete(dir_name: String) -> Result<(), String> {
        let del: Result<(), std::io::Error> = fs::remove_dir_all(dir_name);
        
        match del {
            Ok(()) => return Ok(()),
            Err(err) => return Err("Non-fatal error".to_string()),
        }
    }
}


// impl Drop<T> {
//     fn drop() {
//         cur_holding[0] = None;    
//     }
// }

// impl Open<String> -> Result<(), {
//     let cur_dir: String = // current directory
//     let editors: [String, 4] = ["code, vim, notepad, sublime"];

//     fn open(editor_name: String, file_name:) {
//         if editor_name in editors {
//             // C# console in editor then file name (eg. vim file_name) will open foo.txt with vim
//             fs::File::open(file_name)
//         }
//     }
// }

// impl Search<String> {
//     fn list_dir(root: String>) -> Option<Vec<String>> {
//         let mut q: Vec<String> = vec![root];
//         let mut res: Vec<Vec<String>> = vec![];
        
//         while q.len() > 0 {
//             let mut level: Vec<String> = vec![];

//             for children in fs::read_dir(root)? {
//                 let cur = children?;
//                 match cur {
//                     Err(err) => return Some(res);
//                 }

//                 let cd: String = cur.path();

//                 if cd.is_dir() {
//                     q.push(&cd);
//                 } else {
//                     level.push(cd);
//                 }
//             }

//             res.push(level)
//         }
        
//         if res.len() > 0 {
//             return Some(res)
//         }

//         return None;
//     }

//     fn find_file(find: String) -> Result<bool, String> {
//         match Path::new(find).exists() {
//             true => Ok(true),
//             false => Err("File does not exist or permissions are restricted.".to_string())
//         }
//     }
// }



pub fn working_dir(path: &dyn Display) -> Result<(), String> {
    let cur: Result<PathBuf, std::io::Error> = env::current_dir();
    
    match cur {
        Ok(cur) => {
            path = &cur.display();
            return Ok(())
        },
        Err(err) => Err("Error getting current directory.".to_string())
    }
    
}