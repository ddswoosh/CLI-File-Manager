#[derive(Debug, Clone)]
pub enum Action {
    dummy,
    cd,
    nf,
    df,
    nd,
    dd,
    odd,
    mov,
    grab,
    drop,  
}

#[derive(Debug, Clone)]
pub struct Node {
    op: Action,
    param1: Option<String>,
    param2: Option<String>, 
    next: Option<Box<Node>>,
    prev: Option<Box<Node>>,
}

pub struct List {
    tail: Box<Node>,
    count: usize,  
}

impl Node {
    pub fn new(op: Action, param1: Option<String>, param2: Option<String>) -> Self {
        Node {
            op: op,
            param1: param1,
            param2: param2,
            next: None,
            prev: None,
        }
    }
}

impl List {
    fn init(start: Box<Node>) -> Self {
        List {
            tail: start,
            count: 1
        }
    }

    pub fn add(&mut self, mut node: Box<Node>) {
        if self.count > 0 {
            self.tail.next = Some(node.clone());
            node.prev = Some(self.tail.clone());
            self.tail = node;
            self.count += 1;
       }
    }

    pub fn show(&mut self) {
        while self.count > 1 {
            println!("{:?} {:?} {:?}", self.tail.op, self.tail.param1, self.tail.param2);
            self.count -= 1;
            self.tail = self.tail.prev.clone().unwrap();
        } 
    }
}
pub fn new_tail(node: Box<Node>) {
    
}
pub fn run() -> List {
    let mut dummy: Node = Node::new(Action::dummy, None, None);
    let mut list: List = List::init(Box::new(dummy));

    return list;
}