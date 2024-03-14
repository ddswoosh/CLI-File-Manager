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



struct Node<T, Action> {
    val: T,
    change, T
    action1: Action,
    action2: Option<Action>, 
    next: Option<Box<Node<T, Action>>>,
    prev: Option<Box<Node<T, Action>>>,
}

impl Node<String, Action> {
    fn new(val: String, action1: Action, action2: Option<Action>) -> Self {
        Node {
            val: val,
            action1: action1,
            action2: None,
            next: None,
            prev: None,
        }
    }
}

fn retrieve(idx: usize, node: Option<Box<Node<String, Action>>>) {

    let mut input: String = String::new();
    let mut cur = node;


    for i in 0..idx {
        cur = cur.next;
    }

    println!("{} operations ago, you performed the action {}",idx,cur.val);
    println!("If you would like to return back to this point, please type 'Yes' to stage this reversion.");

    if let ans = io::stdin().read_line(&mut input) == "Yes" {
        stage(&ans);
    }
} 

fn stage(rev_type: String){

   match rev_type {
        Action::Newfile => new_file(),
        Action::Deletefile => delete_file(),
        Action::NewDirectory => new_directory(),
        Action::DeleteDirectory => delete_directory(),
        Action::Move => mov(),
        _ => println!("Cannot revert this function type. New/Delete file | New/Delete Directory | Move, are the only supported functions for reversion.")
   }
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

struct List<T, Node> {
    head: Option<Node>,
    tail: Option<Node>,
    count: T,
    
}

impl List<usize, Node> {
    fn initialize() {
        let mut head = op1;
        let mut tail = dummy;
        let mut count: usize = 0; 
        
    }

    fn add(node: Node) {
        node.prev = tail;
        tail.next = node;
        tail = tail.next;
        *count += 1;
      
    }
    fn remove(idx: usize) [
        if idx == count - 1 {
            tail.prev.next = None;
            tail = tail.prev;
        }
        else {
            let mut cur = &tail;
            let comp = count - idx;
            let mut i: usize = count;

            while i <= comp {
                cur = cur.prev;
                i -= 1;
            }
            cur.prev.next = cur.next;
            cur.next.prev = cur.prev;
        }    
    ]
}


pub fn run() {
    let mut cur_holding Vec<i32> = vec![];

    let op1 = Node::new(String::new(), Action::start);
    let dummy = Node::new(String::new(), Action::start);

}