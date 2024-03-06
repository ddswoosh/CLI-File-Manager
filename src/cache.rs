use std::io;

struct Node<T> {
    val: T,
    next: Option<Box<Node<T>>>,
    prev: Option<Box<Node<T>>>,
}

impl Node<String> {
    fn new(val: String) -> Self {
        Node {
            val: val,
            next: None,
            prev: None
        }
    }

}

fn retrieve(idx: usize, node: Option<Box<Node<String>>>) {

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
    
}

pub fn run() {
    let op1 = Node::new(String::new("History Start"));
}