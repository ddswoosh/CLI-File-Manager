use std::collections::HashMap;

mod integrate;
mod controls;


fn main() {

    let mut hm: &mut HashMap<String, String> = &mut controls::hm();

    integrate::read(&mut hm);
    
}
