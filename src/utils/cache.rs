#[derive(Debug)]
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

#[derive(Debug)]
struct Node {
    op: Action,
    param1: Option<String>,
    param2: Option<String>, 
    next: Option<Box<Node>>,
    prev: Option<Box<Node>>,
}

struct List {
    tail: Box<Node>,
    count: usize,  
}


impl Node {
    fn new(op: Action, param1: Option<String>, param2: Option<String>) -> Self {
        Node {
            op: op,
            param1: param1,
            param2: param2,
            next: None,
            prev: None,
        }
    }

    fn clone(node: Node) -> Self {
        return Self::new(node.op, node.param1, node.param2)
    }
}

impl List {
    fn init(start: Box<Node>) -> Self {
        List {
            tail: start,
            count: 0
        }
    }

    fn add(&mut self, mut node: Box<Node>) {
        node.prev = Some(self.tail);
        self.tail.next = Some(node);
        self.tail = node;
        
        
    }


}

pub fn run() {
    let mut dummy: Node = Node::new(Action::dummy, None, None);
    let mut test1: Node = Node::new(Action::mov, Some("try".to_string()), None);

    let mut start: List = List::init(Box::new(dummy));
    start.add(Box::new(test1));

    // println!("{:?}", start.tail)

}