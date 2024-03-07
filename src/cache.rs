use std::io;

enum Action {
    start,
    mov,
    nfil,
    dfil,
    ndir,
    ddir,
    grab,
    open,
    close,
    search,
}

struct Node<T,Action> {
    val: T,
    action: Action,
    next: Option<Box<Node<T, Action>>>,
    prev: Option<Box<Node<T, Action>>>,
}

impl Node<String, String> {
    fn new(val: String, action: Action) -> Self {
        Node {
            val: val,
            action: action,
            next: None,
            prev: None
        }
    
        &mut count += 1;    

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

fn stage(check: String){
   let mut avail: bool = false;

   let mut u_action: String = check;

   match u_action {
        Action::nfil => avail = true, _;
        Action::dfil => avail = true, _;
        Action::ndir => avail = true,_;
        Action::ddir => avail = true,_;
        Action::mov => avail = true, _;
        _ => println!("Cannot revert this function type. New/Delete file | New/Delete Directory | Move, are the only supported functions for reversion.")
   }
   
   if avail == true {
        // run some fn
   }
   
}

fn move() {

}

fn newDirectory() {

}

fn removeDirectory() {

}

fn newFile() {

}

fn removeFile() {

}

pub fn run() {
    let mut count: usize = 0;

    let op1 = Node::new(String::new("History Start", Action::start));

    let mut head = Some(op1);
    let mut tail = Some(op1);
}