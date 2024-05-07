use std::io;
use std::fs;
use std::env;
use std::path::Path;
use std::path::PathBuf;
use std::fmt::Display;
use std::process::Command;
use std::string::String;
use std::collections::HashMap;

use crate::utils::cache;
use crate::routes::integrate;

pub struct Environment;
pub struct FileArray;
pub struct Mov;
pub struct Fil;
pub struct Dir;
pub struct Open; 
pub struct Search;
pub struct Copy;

pub fn help() -> String {
    return 
"    
nd (create directory) -> nd test
dd (delete empty directory) -> dd test 
odd (delete non-empty directory) -> odd test 
nf (create file) -> nf test python 
df (delete file) -> df test.py 
open (open file in an editor) -> open code test.py 
grab (store file/directory in an array) -> grab test.py 
show (display file in the grab array) -> show 
drop (drop file/directory from array) -> drop 
mov (move file/directory) -> mov test.py {or type holding to use grabbed file} new/test.py 
copy (copy file/directory) -> copy test.py new/test.py 
cd (change directory) -> cd directory or cd back to pop up a level
list (display all items in current directory) -> list
addext (display all items in current directory) -> add javascript .js
added (display all items in current directory) -> add vim ../vim.exe
cache (display 5 most recent nodes to be reverted) -> cache
cache- (display 5 less recent nodes to be reverted) -> cache-
cache+ (display 5 morerecent nodes to be reverted) -> cache+
integer (stage a node for reversion) -> 1
revert (revert the operation stored in the staged node) -> revert
c (clear screen) -> c
".to_string()
}

pub fn file_ext() -> HashMap<String, String> {
    let mut file_map: HashMap<String, String> = HashMap::new();

    file_map.insert("python".to_string(), ".py".to_string());
    file_map.insert("java".to_string(), ".java".to_string());
    file_map.insert("c++".to_string(), ".cpp".to_string());
    file_map.insert("c".to_string(), ".c".to_string());
    file_map.insert("rust".to_string(), ".rs".to_string());
    file_map.insert("text".to_string(), ".txt".to_string());
    file_map.insert("pdf".to_string(), ".pdf".to_string());
    file_map.insert("executable".to_string(), ".exe".to_string());
    file_map.insert("json".to_string(), ".json".to_string());

    return file_map;
}

pub fn file_ext_insert(full: String, ext: String, extentions: &mut HashMap<String, String>) -> &mut HashMap<String, String> {
    extentions.insert(full, ext);

    return extentions;
}

impl FileArray {
    pub fn drop(cur_holding: &mut [Option<PathBuf>; 1]) -> Result<String, String> {
        cur_holding[0] = None;  
        
        return Ok("Success".to_string());
    }

    pub fn grab(name: PathBuf, cur_holding: &mut [Option<PathBuf>; 1]) -> Result<String, String>{
        let mut cur_path: PathBuf = Environment::working_dir().unwrap();
        let found: Result<String, String> = Search::find_file(&name.display().to_string());

        match found {
            Ok(_) => {
                cur_path.push(name);
                cur_holding[0] = Some(cur_path);

                match cur_holding[0] {
                    Some(_) => {
                        return Ok("Success".to_string())
                    },
                    None => return Ok("File may not exist".to_string())
                } 
        },
            Err(_) => return Ok("File may not exist".to_string())
        }
    }

    pub fn show_path(cur_holding: &[Option<PathBuf>; 1]) -> Result<String, String> {
        if cur_holding[0].is_some() {
            return Ok(cur_holding[0].as_ref().unwrap().display().to_string());
        }

        return Ok("None".to_string());
    }

    pub fn show_node(cur_holding_node: &[Option<cache::Node>; 1]) -> Option<&cache::Node> {
        if cur_holding_node[0].is_some() {
            return Some(&cur_holding_node[0].as_ref().unwrap());
        }

        return None;
    }
}

impl Mov {
    pub fn mov(from: String, to: String, list: &mut cache::List) -> Result<String, String> {
        let mut cur_path: PathBuf = Environment::working_dir().expect("Err");
        cur_path.push(from.clone());
        
        let attempt: Result<(), std::io::Error> = fs::rename(cur_path, to.clone());

        match attempt {
            Ok(_) => {
                let mut node: cache::Node = cache::Node::new(cache::Action::move_file, Some(from), Some(to));
                cache::List::add(list, Box::new(node));

                return Ok("Success".to_string());
            },
            Err(_) => return Ok("Destination is not valid, please try again.".to_string())
        }
    }
}

impl Copy {
    pub fn copy(from: String, to: String) -> Result<String, String> {
        let result: Result<u64, std::io::Error> = fs::copy(from, to);
        
        match result {
            Ok(_) => return Ok("Success".to_string()),
            Err(_) => return Err("Copy failed".to_string())
        }
    }
}

impl Fil {
    pub fn new_file(file_name: String, file_ext: String, hm: &mut HashMap<String, String>, list: &mut cache::List) -> Result<String, String> {
        if hm.get(&file_ext).is_some() {
            let ext: &String = hm.get(&file_ext).unwrap();
            let name: String = file_name + &ext;

            fs::File::create(name.clone());

            let mut node: cache::Node = cache::Node::new(cache::Action::new_file, Some(name), None);

            cache::List::add(list, Box::new(node));

            return Ok("Success".to_string());

        } else {
            return Ok("Your chosen file type does not yet exist. To create this extension, please type 
            'addext, full extension name, extension' (i.e. addext python .py).".to_string());
        } 
    } 

    pub fn delete_file(name: String, path: PathBuf, list: &mut cache::List) -> Result<String, String> {
        let success = fs::remove_file(name.clone());
        
        match success {
            Ok(_) => {
                let mut node: cache::Node = cache::Node::new(cache::Action::delete_file, Some(name), None);

                cache::List::add(list, Box::new(node));

                return Ok("Success".to_string());
            },
            Err(_) => return Ok("Check the file exists or the your spelling is correct.".to_string())
        }
    }
}

impl Dir {
    pub fn new_directory(dir_name: String, mut path: PathBuf, list: &mut cache::List) -> Result<String, String> {
        for i in dir_name.chars() {
            if i == '.' {
                return Ok("Cannot use {.} in directories, this is only available for file extentions".to_string());
            }
        }

        let success: Result<(), std::io::Error> = fs::create_dir(dir_name.clone());
        
        match success {
            Ok(_) => {
                let mut node: cache::Node = cache::Node::new(cache::Action::new_directory, Some(dir_name), None);
                cache::List::add(list, Box::new(node));

                return Ok("Success".to_string());
            },
            Err(_) => return Ok("Path already exists".to_string())
        }
    } 

    pub fn delete_directory(dir_name: String, mut path: PathBuf, list: &mut cache::List) -> Result<String, String> {
        let is_path: Result<String, String> = Search::find_file(&dir_name);

        match is_path {
            Ok(_) => {
                path.push(&dir_name);
        
                match path.is_dir() {
                    true => {
                        let del: Result<(), std::io::Error> = fs::remove_dir(dir_name.clone());
                
                        match del {
                            Ok(_) => {
                                let mut node: cache::Node = cache::Node::new(cache::Action::delete_directory, Some(dir_name), None);
                                cache::List::add(list, Box::new(node));

                                return Ok("Success".to_string());
                            },
                            Err(_) => return Ok("The directory is not empty, please remove all contents or type 'odd {directory name}' if you want to force delete all child content".to_string()),
                        }
                    },

                    false => return Ok("This is not a directory".to_string())
                }
            },
            Err(_) => Ok("Directory not found".to_string())
        }
    }

    pub fn override_delete(dir_name: String, mut path: PathBuf, list: &mut cache::List) -> Result<String, String> {
        let is_path: Result<String, String> = Search::find_file(&dir_name);

        match is_path {
            Ok(_) => {
                path.push(&dir_name);
        
                match path.is_dir() {
                    true => {
                        let del: Result<(), std::io::Error> = fs::remove_dir_all(dir_name.clone());
                
                        match del {
                            Ok(()) => {
                                let mut node: cache::Node = cache::Node::new(cache::Action::override_delete_directory, Some(dir_name), None);
                                cache::List::add(list, Box::new(node));

                                return Ok("Success".to_string());
                            },
                            Err(_) => return Ok("Fatal error".to_string()),
                        }
                    },

                    false => return Ok("This is not a directory".to_string())
                }
            },
            Err(_) => Ok("Directory not found".to_string())
        }
    }
}

impl Open {
    pub fn open(file_name: String, editor_name: String, editors: &mut HashMap<String, String>) -> Result<String, String> {
        let is_file: Result<String, String> = Search::find_file(&file_name);

        match is_file {
            Ok(_) => {
                let mut cur_path: PathBuf = Environment::working_dir().expect("Err");
                cur_path.push(&file_name);
    
                if editors.get(&editor_name).is_some() {
                    let status = Command::new(editors.get(&editor_name).unwrap()).arg(cur_path).status();
    
                    match status {
                        Ok(_) => return Ok("Success".to_string()),
                        Err(_) => return Ok("Could not open editor".to_string())
                    };     
                } else {
                    return Ok("Editor not found, if you want to add your own editor, type help and follow the add editor instructions".to_string())
                }   
            }
            Err(_) => return Ok("This file does not exist".to_string())
        }     
    }

    pub fn add_editor(editor_name: String, path: String, editors: &mut HashMap<String, String>) -> Result<String, String> {
        let success: Option<String> = editors.insert(editor_name, path);

        match success { 
            Some(_) => return Ok("Success".to_string()),
            None => return Ok("Error adding editor to list".to_string())
        }
    }
}

impl Search {
    pub fn list_dir() -> Result<String, String> {
        let mut children: Vec<PathBuf> = vec![];
        let mut cur: PathBuf = Environment::working_dir().unwrap();
       
        if cur.is_dir() {
            for child in fs::read_dir(cur).unwrap() {
            
                let child: fs::DirEntry = child.unwrap();
                let mut path: PathBuf = child.path();

                let mut temp: String = path.display().to_string();
                temp = temp[..temp.len()].to_string();
                
                if path.is_dir() {
                    temp += " -> Directory";
                    children.push(temp.into());

                } else {
                    temp += " -> File";
                    children.push(temp.into());
                }
            }
        } else {
            return Ok("This file name is not of type: Directory".to_string());
        }

        if children.len() == 0 {
            return Ok("Directory has no children".to_string());

        } else {
            let mut res: String = String::new();

            for i in 0..children.len() {
                let mut temp: String = children[i].display().to_string();
                let mut j: usize = 0;

                if temp.chars().rev().nth(0) == Some('y') {
                    for i in temp.chars().rev() {
                        if i == '\\' && j > 14 {
                            break;
                        }
                        j += 1;
                    }
                 } else {
                    for i in temp.chars().rev() {
                        if i == '\\' && j > 9 {
                            break;
                        }
                        j += 1;
                    }
                }

                j = temp.len() - j;

                for i in 0..temp.len() {
                    if i == j {
                        temp = temp[j..].to_string();
                        break;
                    }
                }

                res += &(temp);
                res += "\n\n";
            }
        
            return Ok(res);
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

    pub fn change_dir(change: String, list: &mut cache::List) -> Result<String, String> {
        let mut cur: PathBuf = Self::working_dir().unwrap();

        if change == "back".to_string() { 
            let mut temp_vec: Vec<String> = vec![];
            let mut new_path: String = String::new();
            let mut end_num: usize = 0;

            for i in cur.display().to_string().chars() {
                temp_vec.push(i.to_string());
            }

            for i in (0..temp_vec.len()-1).rev() {
                if temp_vec[i] == "\\" {
                    *&mut end_num = i;
                    break;          
                }
            }
            
            for i in 0..end_num {
                new_path += &temp_vec[i];
            }

            let success: Result<(), std::io::Error> = env::set_current_dir(new_path.clone());

                    match success {
                        Ok(_) => return Ok("Success".to_string()),
                        Err(_) => return Ok("Error changing directory".to_string())
                    }

        } else {
            let exists: Result<String, String> = Search::find_file(&change);

            match exists {
                Ok(_) => {
                    cur.push(change);

                    let success: Result<(), std::io::Error> = env::set_current_dir(cur);

                    match success {
                        Ok(_) => return Ok("Success".to_string()),
                        Err(_) => return Ok("Error changing directory".to_string())
                    }
                }
                Err(_) => return Ok("Change to directory does not exsist within the scope of your current directory".to_string())
            }
        }
    }
}
