// use std::io;
// use std::cmp::Ordering;
use std::io::stdin;

fn main() {
    println!("This is a menu, you can enter:");
    println!("1.help");
    println!("2.test");
    println!("3.quit");
    loop {
        println!("please enter a message:");
        let mut s = String::new();
        stdin().read_line(&mut s).unwrap();
        match &s[..s.len() - 2] {
            "quit" => break,
            "help" => println!("help message"),
            "list" => println!("test message"),
            _ => println!("unknow command"),
        }
    }
}
