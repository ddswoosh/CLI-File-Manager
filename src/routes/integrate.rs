use std::fs;
use std::path::PathBuf;
use std::fmt::Display;
use std::collections::HashMap;
use std::str::FromStr;

use crate::utils::cache;
use crate::controllers::controls;
use crate::controllers::revert;

pub fn read(
    extensions: &mut HashMap<String, String>, cur_holding_path: &mut [Option<PathBuf>; 1], 
    cur_holding_node: &mut [Option<cache::Node>; 1], editors: &mut HashMap<String, String>, cur_command: &mut String, 
    list: &mut cache::List, num_node: &mut u8, shift: &mut i8, cli_user_path: &String
) -> String {

    let mut path: PathBuf = controls::Environment::working_dir().expect("Non-fatal error");
    let write_to: String = format!("{}\\dump\\command.txt", &cli_user_path);
    let cin: String = fs::read_to_string(write_to).map_err(|e| e.to_string()).expect("Failed to read from CLI");
    
    *cur_command = cin.clone();

    let vec: Vec<&str> = cin.split_whitespace().collect();
    let mut command: &str = "";
    let mut name: &str = "";
    let mut ext: &str = "";

    if &vec[0] == &"-help" {
        return controls::help();
    }

    else {
        if vec.len() == 3 {
            command = &vec[0];
            name = &vec[1];
            ext = &vec[2];

            match command {
                "nf" => return controls::Fil::new_file(name.to_string(), ext.to_string(), extensions, list).unwrap(),
                "open" => return controls::Open::open(ext.to_string(), name.to_string(), editors).unwrap(),
                "added" => return controls::Open::add_editor(name.to_string(), ext.to_string(), editors).unwrap(),
                "copy" => return controls::Copy::copy(name.to_string(), ext.to_string()).unwrap(),
                "addext" => {
                    controls::file_ext_insert(name.to_string(), ext.to_string(), extensions);
                    return "success".to_string();
                },
                "mov" => {
                    if name == "holding".to_string(){
                        return controls::Mov::mov(cur_holding_path[0].clone().unwrap().display().to_string(), ext.to_string(), list).unwrap()
                        } 
                        return controls::Mov::mov(name.to_string(), ext.to_string(), list).unwrap()
                },
                _ => return "Command not accepted, please type -help to see the list of all commands.".to_string()
        }
        
        } else if vec.len() == 2 {
            command = &vec[0];
            name = &vec[1];

            match command {
                "nd" => return controls::Dir::new_directory(name.to_string(), path, list).unwrap(),
                "dd" => return controls::Dir::delete_directory(name.to_string(), path, list).unwrap(),
                "odd" => return controls::Dir::override_delete(name.to_string(), path, list).unwrap(),
                "df" => return controls::Fil::delete_file(name.to_string(), path, list).unwrap(),
                "grab" => return controls::FileArray::grab(name.into(), cur_holding_path).unwrap(),
                "cd" => return controls::Environment::change_dir(name.to_string(), list).unwrap(),
                _ => return "Command not accepted, please type -help to see the list of all commands.".to_string()
            }

        } else if vec.len() == 1 {
            command = &vec[0];

            if command == "wd" {
                return controls::Environment::working_dir().expect("Fatal error").display().to_string();
            }

            match command {
                "cache" => {
                    *num_node = list.count.clone() - 1;
                    let mut temp_list = list.clone();

                    return cache::List::display_cache(&mut temp_list, num_node, shift)
                },

                "cache-" => {
                    *shift -= 5;
                    *num_node = list.count.clone() - 1;
                    let mut temp_list = list.clone();

                    return cache::List::display_cache(&mut temp_list, num_node, shift)
                },

                "cache+" => {
                    *shift += 5;
                    *num_node = list.count.clone() - 1;
                    let mut temp_list = list.clone();

                    return cache::List::display_cache(&mut temp_list, num_node, shift)
                },
                "drop" => return controls::FileArray::drop(cur_holding_path).unwrap(),
                "show" => return controls::FileArray::show_path(cur_holding_path).unwrap(),
                "list" => return controls::Search::list_dir().unwrap(),
                "revert" => return revert::receive_node(cur_holding_node, list, extensions),
                num => {
                    if num.parse::<u8>().is_ok() { 
                        let mut temp: u8 = FromStr::from_str(command).unwrap();
                        let mut temp_list: cache::List = list.clone();
                    
                        if (temp as char).is_digit(36) == false {
                            
                            let wrapped_node: Option<&cache::Node> = cache::List::get_node(&mut temp_list, temp);

                            match wrapped_node {
                                Some(_) => {
                                    let unwrapped_node: &cache::Node = wrapped_node.unwrap();
                                    let temp_holding: cache::Node = unwrapped_node.clone();
                                    let mut res: String = String::new();

                                    res += &cache::Action::to_string(&unwrapped_node.op);
                                    res += " - {";
                                    
                                    if unwrapped_node.param1.is_some() {
                                        res += &unwrapped_node.param1.clone().unwrap();
                                    }

                                    if unwrapped_node.param2.is_some() {
                                        res += "--";
                                        res += &unwrapped_node.param2.clone().unwrap();
                                    }
                                    res += "}";

                                    cur_holding_node[0] = Some(temp_holding);
                                    
                                    return res;
                                }
                                
                                None => return "The node you selected is not in the cache".to_string()
                            }

                        } else {
                            return "Please enter an integer between 1 and 255".to_string();
                        } 
                    } else {
                        return "Command not accepted, please type -help to see the list of all commands".to_string();
                    }
                    
                }    
            }
        } else {
            return "Command not accepted, please type -help to see the list of all commands.".to_string();
        }
    }
}
