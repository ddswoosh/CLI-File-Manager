mod controls;

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