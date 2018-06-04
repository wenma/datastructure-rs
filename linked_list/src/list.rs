

use std::rc::Rc;

#[derive(Debug)]
pub struct List {
    head: Rc<Link>
}


#[derive(Debug)]
enum Link {
    Empty,
    More(Box<Node>)
}

#[derive(Debug)]
struct Node {
    data: i32,
    next: Rc<Link>
}


impl List {
    pub fn new() -> Self {
        List {
            head: Rc::new(Link::Empty)
        }
    }

    pub fn insert_head(&mut self, elem: i32) {

        if (Rc::get_mut(&mut self.head)).is_some() {
            self.head = Rc::new(Link::More(Box::new(Node {
                    data: elem,
                    next: self.head.clone()
                })));

        } else {
            self.head = Rc::new(Link::More(Box::new(Node {
                    data: elem,
                    next: Rc::new(Link::Empty)
            })));
        }
        
    }

    pub fn print(&self) {
        println!("{:?}", self.head);
    }
}