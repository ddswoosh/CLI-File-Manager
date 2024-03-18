use std::io;
use std::fs;
use std::env;
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
    file_map.insert("python", ".py");
    file_map.insert("c++", ".cpp");
    file_map.insert("c", ".c");
    file_map.insert("rust", ".rs");
    file_map.insert("text", ".txt");
    file_map.insert("pdf", ".pdf");
    file_map.insert("exe", ".exe");
    file_map.insert("json", ".json");

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
    fn new_directory(dir_name: String) -> Result<(), &'static str> {
        let cur_dir: String = //windows current directory;
        let success: Result<(), std::io::Error> = fs::create_dir(dir_name);
        
        match success {
            Ok(()) => return Ok(()),
            Err(err) => return Err("Path at {} already exists", dir_name),
        }

    } 
}

impl Ddir<String> {
    fn delete_directory(dir_name: String) {
        let cur_dir: String = //windows current directory;

        let del: Result<(), std::io::Error> = fs::remove_dir(dir_name);

        match del {
            Ok(()) => return Ok(()),
            Err(err) => return Err("The directory is not empty, please remove all contents or type 'Override' if you want to force
            delete all child content. Type cancel to exit this function."),
        }

        // read next input

        if &mut input == "Override" {
            return override_delete(dir_name);
        } else {
            // return to main
        }
    }

    fn override_delete(dir_name: String) -> Result<()> {
        fs::remove_dir_all(dir_name)?;
        return Ok(())
    }
}

impl Drop<T> {
    fn drop() {
        cur_holding[0] = None;    
    }
}

impl Open<String> -> Result<(), {
    let cur_dir: String = // current directory
    let editors: [String, 4] = ["code, vim, notepad, sublime"];

    fn open(editor_name: String, file_name:) {
        if editor_name in editors {
            // C# console in editor then file name (eg. vim file_name) will open foo.txt with vim
            fs::File::open(file_name)
        }
    }
}

impl Search<String> {
    fn search(root: String, find: String) -> Result<bool>{
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

