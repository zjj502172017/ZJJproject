include!("linktable.rs");

fn main() {
    let mut link = LinkedList::new();
    link.push(5);
    link.push(4);
    link.push(3);
    link.push(2);
    link.push(1);
    println!("{link}");
    link.pop();
    link.reverse();
    println!("{link}");
}
