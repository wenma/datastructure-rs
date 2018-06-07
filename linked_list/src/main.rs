

mod list;

use list::List;

fn main() {
    let mut list: List = List::new();


    for i in 0..10 {
    	list.insert_head(i);
    }

    list.print_list();

}
