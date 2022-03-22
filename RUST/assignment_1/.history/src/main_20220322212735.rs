// use std::io;
// use std::cmp::Ordering;
use std::io::stdin;

fn main() {
    println!("This is a menu, you can enter:");
    println!("1.test");
    println!("2.test");
    println!("3.quit");
    loop {
        let mut s = String::new();
        print!("please enter a message:");
        stdin().read_line(&mut s).unwrap();
        match &s[..s.len() - 2] {
            "quit" => break,
            "help" => println!("help msg"),
            "list" => println!("list msg"),
            _ => println!("unknow command"),
        }
    }
}
