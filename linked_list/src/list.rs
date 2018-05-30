



#[derive(Debug)]
pub struct List {
	head: Link
}

#[derive(Debug)]
enum Link {
	Empty,
	More(Box<Node>)
}

#[derive(Debug)]
struct Node {
	data: i32,
	next: Link
}


impl List {
	pub fn new() -> Self {
		List {
			head: Link::Empty
		}
	}

	pub fn insert_head(&mut self, elem: i32) {

		match self.head {
			Link::Empty => {
				self.head = Link::More(Box::new(Node {
					data: elem,
					next: Link::Empty
				}));
			},
			Link::More(a) => {
				self.head = Link::More(Box::new(Node {
					data: elem,
					next: Link::More(a)
				}));
			}
		}
		
	}

	pub fn insert_tail(&mut self, elem: i32) {

	}
}