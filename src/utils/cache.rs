// struct Node<T, Action> {
//     val: T,
//     change, T
//     action1: Action,
//     action2: Option<Action>, 
//     next: Option<Box<Node<T, Action>>>,
//     prev: Option<Box<Node<T, Action>>>,
// }

// impl Node<String, Action> {
//     fn new(val: String, action1: Action, action2: Option<Action>) -> Self {
//         Node {
//             val: val,
//             action1: action1,
//             action2: None,
//             next: None,
//             prev: None,
//         }
//     }
// }

// struct List<T, Node> {
//     head: Option<Node>,
//     tail: Option<Node>,
//     count: T,  
// }

// impl List<usize, Node> {
//     fn initialize() {
//         let mut head = op1;
//         let mut tail = dummy;
//         let mut count: usize = 0;  
//     }

//     fn add(node: Node) {
//         node.prev = tail;
//         tail.next = node;
//         tail = tail.next;
        
//         *count += 1;   
//     }

//     fn remove(idx: usize) {
//         if idx == count - 1 {
//             tail.prev.next = None;
//             tail = tail.prev;
//         }
//         else {
//             let mut cur = &tail;
//             let comp = count - idx;
//             let mut i: usize = count;

//             while i <= comp {
//                 cur = cur.prev;
//                 i -= 1;
//             }
//             cur.prev.next = cur.next;
//             cur.next.prev = cur.prev;
//         }   

//         *count -= 1;
//     }
// }

// pub fn run() {
//     let mut cur_holding: [Option<String>; 1] = [None];

//     let op1 = Node::new(String::new(), Action::start);
//     let dummy = Node::new(String::new(), Action::start);

// }