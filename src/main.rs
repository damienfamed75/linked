pub mod first;
pub mod second;

use first::List;

fn main() {
    let mut l = List::new();
    l.push(0);
    l.push(1);
    l.push(2);

    println!("{:?}", l.pop());
    println!("{:?}", l.pop());
    println!("{:?}", l.pop());
}
