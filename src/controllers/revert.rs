use std::path::PathBuf;
use std::collections::HashMap;

use crate::controllers::controls;
use crate::utils::asyncfn;
use crate::utils::cache;

pub fn receive_node(cur_holding_node: &mut [Option<cache::Node>; 1], list: &mut cache::List, extensions: &mut HashMap<String, String> ) -> String {
    let node: Option<&cache::Node>  = cur_holding_node[0].as_ref();
    let mut path: PathBuf = controls::Environment::working_dir().expect("Non-fatal error");

    if !node.is_some() {
        return "Please select a node to stage before proceeding to reversion".to_string();
    }

    let temp = node.unwrap().clone();

    match temp.op {  
        cache::Action::dummy => return "Err".to_string(),
        cache::Action::new_directory => return controls::Dir::delete_directory(temp.param1.expect("Empty parameter"), path, list).unwrap(),
        cache::Action::delete_directory => return controls::Dir::new_directory(temp.param1.expect("Empty parameter"), path, list).unwrap(),
        cache::Action::override_delete_directory => return controls::Dir::new_directory(temp.param1.expect("Empty parameter"), path, list).unwrap(),
        cache::Action::new_file => return controls::Fil::delete_file(temp.param1.expect("Empty parameter"), path, list).unwrap(),
        cache::Action::delete_file => {

            let iterator = temp.clone().param1.expect("Empty parameter");
            let mut name: String = String::new();
            let mut short_ext: String = String::new();
            let mut full_ext: String = String::new();

            for i in 0..iterator.len() {
                if iterator.chars().nth(i).unwrap() == '.' {
                    short_ext += &iterator[i..];
                    name += &iterator[..i]
                }
            }

            for (i,j) in extensions.iter() {
                if *j == short_ext {
                    full_ext += i; 
                }
            }

            return controls::Fil::new_file(name, full_ext, extensions, list).unwrap();
        },
        cache::Action::move_file => {
            let mut temp_buf: PathBuf = controls::Environment::working_dir().unwrap();
            let mut full_path: String = String::from(temp_buf.display().to_string());
            let mut j: usize = 0;

            for i in temp.param1.clone().expect("Empty parameter").chars().rev() {
                if i == '\\' {
                    break;
                }

                j += 1;
            }

            let part_path: String = temp.param1.clone().unwrap()[..j].to_string();
            let mut temp_vec: Vec<char> = vec![];
            
            for i in part_path.chars() {
                temp_vec.push(i);
            }

            full_path += &"\\";

            for i in temp_vec.iter() {
                if *i == '/' {
                    full_path += &"\\";

                } else {
                    full_path += &i.to_string();
                }
            }

            return controls::Mov::mov(temp.param2.expect("Empty parameter"), full_path, list).unwrap()     
        }  
    }
}