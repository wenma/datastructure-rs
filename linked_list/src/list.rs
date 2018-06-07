

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

        let mut next = Rc::new(Link::Empty);

        match *(self.head) {
            Link::Empty => {},
            Link::More(_) => next = self.head.clone()
        }

        self.head = Rc::new(Link::More(Box::new(Node {
            data: elem,
            next: next
        })));
        
    }


    pub fn print_list(&self) {
        let mut pointer = self.head.clone();
        let mut next: Rc<Link>;
        loop {
            match *pointer {
                Link::Empty => {
                    println!("NULL");
                    break;
                },
                Link::More(ref b) => {
                    print!("{:?}", (**b).data);
                    next = (**b).next.clone();
                }
            }

            print!(" -> ");
            pointer = next;
        }
    }
}