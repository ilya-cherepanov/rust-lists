use crate::first::List;

mod first;

fn main() {
    let mut list = List::<i32>::new();
    list.push(10);
    list.push(20);
    list.push(30);
    println!("{:?}", list);
    println!("{:?}", list.pop());
    println!("{:?}", list.pop());
    println!("{:?}", list.pop());
    println!("{:?}", list.pop());
}
