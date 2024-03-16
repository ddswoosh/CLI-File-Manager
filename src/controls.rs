use std::io;
use std::collections::HashMap;

enum Action {

    Start,
    Grab,
    Move,
    Newfile,
    Deletefile,
    NewDirectory,
    DeleteDirectory,
    Drop,
    Open,
    Search,
}

struct Grab<T> {
    grab_name: T,
}

struct Mov<T> {
    init_path: T,
    end_path: T,
}

struct Nfil<T> {
    file_name: T,
    file_type: T,
}

struct Dfil<T> {
    file_name: T,
}

struct Ndir<T> {
    directory_name: T,
}

struct Ddir<T> {
    directory_name: T,
}

struct Drop<T> {
    held: T,
}

struct Open<T> {
    editor_name: T,
    file_name: T,
}
struct Search<T> {
    root_dir: T,
    key: T,
}

fn hm() -> HashMap<K, V> {
    let mut file_map: HashMap<K, V> = HashMap::new();
    file_map.insert("Python", ".py");
    file_map.insert("C++", ".cpp");
    file_map.insert("C", ".c");
    file_map.insert("Rust", ".rs");
    file_map.insert("Text", ".txt");
    file_map.insert("PDF", ".pdf");
    file_map.insert("Exe", ".exe");
    file_map.insert("JSON", ".json");

    return file_map;
}

fn hm_insert(full: String, ext: String) {
    let mut x: HashMap<K, V> = hm();
    &file_map.insert(full, ext);
    
    println!("If your entered file type is not supported in Windows, the file may not open as intended.");
}

impl Grab<String> {
    fn grab(name: String) {
        cur_holding[0] = name;    
    }
}

impl Move<String> {
    fn mov(from: String, to: String, name: String) {
        let x: String = grab(name);

        if search(to).is_some() {

            // move file with windows native function

            let node = Node::new(name, from, to);
            List::add(node);
        } else {
            println!("The path {} is not found. Please create this directory before you move the file.", to);
        }
        
    }
}

impl Nfil<String> {
    fn new_file(file_name: String, file_ext: String) {
        let cur_dir: String = //windows current directory;

        if hm.get(&file_ext).is_some() {
            if !file_name in cur_dir.children {
                // cur.dir add new file, e.g. ("python main" == create main.py)
            }
        } 
    } 
}

impl Dfil<String> {
    fn delete_file(file_name: String) {
        let cur_dir: String = //windows current directory;

        if !file_name in cur_dir.children {
            println!("File does not exist in this directory");
        } else {
            // os.remove(file_name);
        }
    }
}

impl Ndir<String> {
    fn new_directory(dir_name: String) {
        let cur_dir: String = //windows current directory;

        if !cur_dir in cur_dir.children {
            // cur.dir add new dir
        }
    } 
}

impl Ddir<String> {
    fn delete_directory(dir_name: String) {
        let cur_dir: String = //windows current directory;

        if !dir_name in cur_dir.children {
            println!("Folder does not exist in this directory");
        } else {
            // os.remove(dir_name);
        }
    }
}

impl Drop<T> {
    fn drop() {
        cur_holding[0] = None;    
    }
}

impl Open<String> {
    let editors: [String, 4] = ["code, vim, notepad, sublime"];

    fn open(editor_name: String, file_name:) {
        if editor_name in editors {
            // windows open file with said editor
        }
    }
}

impl Search<String> {
    fn search(root: String, find: String) -> Option<bool>{
        let mut q: Vec<i32> = vec![root];
        
        while q.len() > 1{
            let mut level: Vec<i32> = vec![];

            for children in root./*windows fn to get dir children */ {
                level.push(children);
            }

            for i in level.iter() {
                return search(i, find);
            }

        if root == find {
            return Some(true)
        }
    }
        return None;        
    }
}

