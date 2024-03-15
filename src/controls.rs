use std::io;

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
    cur_path: T,
}

struct Dfil<T> {
    // Must be in current directory
    file_name: T,
}

struct Ndir<T> {
    // Creates in the current directory
    directory_name: T,
}

struct Ddir<T> {
    // Directory must be empty or files move to dump
    directory_name: T,
}

struct Drop<Grab> {
    // Will only allow if grab contains a file/folder
    drop_name: Grab,
}

struct Open<T> {
    // Name of the app used to open the file type (e.g. vim)
    app_name: T,
    file_name: T,
    file_type: T
}

struct Search<T> {
    // Look for a file starting from the user defined root directory. DFS Tree traversal.
    root_dir: T,
    key: T,
}

impl Grab<String> {
    fn move{name: String} {
        cur_holding.push(name);    
    }
}


impl Move<String> {
    fn move{from: String, to: String, name: String} {
        let x: String = grab(name);

        if let Some(exists).is_some() = search(to) {
            // move file with windows fun
            let node = Node::new(name, from, to);
        }
        
    }
}
