

mod list;

use list::List;

fn main() {
    let mut list: List = List::new();

    list.insert_head(1);
    list.insert_head(2);
    list.insert_head(3);
    list.insert_head(4);

    list.print();

}
