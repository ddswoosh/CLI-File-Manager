#[derive(Debug, Clone)]
enum Action {
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

    fn show(&mut self) {
        while self.count > 1 {
            println!("{:?}", self.tail.op);
            self.count -= 1;
            self.tail = self.tail.prev.clone().unwrap();
        } 
    }
}

pub fn run() {
    let mut dummy: Node = Node::new(Action::dummy, None, None);
    let mut list: List = List::init(Box::new(dummy));

    let mut one: Node = Node::new(Action::mov, None, None);

    List::add(&mut list, Box::new(one));
    

    List::show(&mut list);

}