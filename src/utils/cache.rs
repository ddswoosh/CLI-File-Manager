#[derive(Debug, Clone)]
pub enum Action {
    dummy,
    change_directory,
    new_file,
    delete_file,
    new_directory,
    delete_directory,
    override_delete_directory,
    move_file,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub op: Action,
    pub param1: Option<String>,
    pub param2: Option<String>, 
    next: Option<Box<Node>>,
    prev: Option<Box<Node>>,
}

pub struct List {
    tail: Box<Node>,
    count: u8,  
}

impl ToString for Action {
    fn to_string(&self) -> String {
        match self {
            Action::dummy => return "dummy".to_string(),
            Action::move_file => return "move_file".to_string(),
            Action::change_directory => return "change_directory".to_string(),
            Action::new_file => return "new_file".to_string(),
            Action::delete_file => return "delete_file".to_string(),
            Action::new_directory => return "new_directory".to_string(),
            Action::delete_directory => return "delete_directory".to_string(),
            Action::override_delete_directory => return "override_delete_directory".to_string(),
        }
    }
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

    pub fn display_cache(&mut self, num_node: &mut u8) -> String {
        let mut res: String = String::new();
        *num_node = self.count - 1;

        while self.count >= *num_node && self.count > 1 {
            if self.tail.param1.is_some() && self.tail.param2.is_some() {
                res += &self.count.to_string();
                res += " - {";
                res += &Action::to_string(&self.tail.op);
                res += "-->";
                res += &self.tail.param1.clone().unwrap();
                res += "-->";
                res += &self.tail.param2.clone().unwrap();
                res += "}   ";
            } else {
                res += &self.count.to_string();
                res += " - {";
                res += &Action::to_string(&self.tail.op);
                res += "-->";
                res += &self.tail.param1.clone().unwrap();
                res += "}   ";
            }

            self.count -= 1;
            self.tail = self.tail.prev.clone().unwrap();
        } 
        
        return res;
    }    

    pub fn get_node(&mut self, num: u8) -> Option<&Node> {
        let mut temp: u8 = self.count.clone();

        while temp != num && temp > 0 {
            temp -= 1;
            if self.tail.prev.is_some() {
                self.tail = self.tail.prev.clone().unwrap();
            } else {
                return None;
            }
        }

        return Some(&self.tail);
    }
}

pub fn run() -> List {
    let mut dummy: Node = Node::new(Action::dummy, None, None);
    let mut list: List = List::init(Box::new(dummy));

    return list;
}