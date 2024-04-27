use std::path::PathBuf;

use crate::controllers::controls;
use crate::utils::asyncfn;
use crate::utils::cache;

fn receive_node(cur_holding_node: &[Option<&cache::Node>; 1], list: &mut cache::List) -> String {
    let node: Option<&cache::Node>  = cur_holding_node[0];
    let mut path: PathBuf = controls::Environment::working_dir().expect("Non-fatal error");

    if !node.is_some() {
        return "Please select a node before proceeding to reversion".to_string();
    }

    match node.unwrap().op {
        
        cache::Action::new_directory => return controls::Dir::delete_directory(node.unwrap().param1.expect("temp"), path, list).unwrap(),

        
    }
}