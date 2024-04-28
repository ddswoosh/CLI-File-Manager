use std::path::PathBuf;

use crate::controllers::controls;
use crate::utils::asyncfn;
use crate::utils::cache;

pub fn receive_node(cur_holding_node: &mut [Option<cache::Node>; 1], list: &mut cache::List) -> String {
    let node: Option<&cache::Node>  = cur_holding_node[0].as_ref();
    let mut path: PathBuf = controls::Environment::working_dir().expect("Non-fatal error");

    if !node.is_some() {
        return "Please select a node before proceeding to reversion".to_string();
    }

    match node.unwrap().op {  
        cache::Action::dummy => return "temp".to_string(),
        cache::Action::new_directory => return "temp".to_string(),
        cache::Action::delete_directory => return "temp".to_string(),
        cache::Action::override_delete_directory => return "temp".to_string(),
        cache::Action::change_directory => return "temp".to_string(),
        cache::Action::new_file => return "temp".to_string(),
        cache::Action::delete_file => return "temp".to_string(),
        cache::Action::move_file => return "temp".to_string(),       
    }
}