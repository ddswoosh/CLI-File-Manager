use std::fs;

mod cache;
mod controls;

fn retrieve(idx: usize, node: Option<Box<Node<String, Action>>>) {

    let mut input: String = String::new();
    let previous_op_idx: usize = &count - idx;
    let mut cur = tail;

    let mut i: i32 = &count

    while i <= previous_op_idx {
        cur = cur.prev;
        i -= 1;
    }
    

    println!("{} operations ago, you performed the action {}",idx,cur.change);
    println!("If you would like to return back to this point, please type 'Yes' to stage this reversion.");

    if io::stdin().read_line(&mut input) == "Yes" {
        stage(&cur.change);
    }
} 

fn stage(rev_type: String){

    match rev_type {
         Action::Newfile => Controls::rev_new_file(),
         Action::Deletefile => Controls::rev_delete_file(),
         Action::NewDirectory => Controls::rev_new_directory(),
         Action::DeleteDirectory => Controls::rev_delete_directory(),
         Action::Move => Controls::rev_mov(),
         _ => println!("Cannot revert this function type. New/Delete file | New/Delete Directory | Move, are the only supported functions for reversion.")
    }
 }
 
 

struct RevMov<T> {
    init_path: T,
    end_path: T,
}

struct RevNfil<T> {
    file_name: T,
    file_type: T,
}

struct RevDfil<T> {
    file_name: T,
}

struct RevNdir<T> {
    directory_name: T,
}

struct RevDdir<T> {
    directory_name: T,
}

impl RevMove<String> {
    fn rev_mov(node: Node) {
        let from: String = node.action2;
        let to: String = node.action2;

        if search(to).is_some() {

            // move file with windows native function

            let node = Node::new(name, from, to);
            List::add(node);
        } else {
            println!("Rversion is not possible, please make sure the destination path of the reversion exists.", to);
        }
    }
}
    
impl RevNfil<String> {
    fn rev_new_file(file_name: String) {
        delete_file(file_name);
    } 
} 


impl RevDfil<String> {
    fn rev_delete_file(file_name: String) {
        // file name .strip() file_ext = (.py, .rs)
        new_file(file_name, file_ext);
    }
}

impl Ndir<String> {
    fn rev_new_directory(dir_name: String) {
        delete_directory(dir_name);
    } 
}

impl Ddir<String> {
    fn rev_delete_directory(dir_name: String) {
        new_directory(dir_name);
    }
}