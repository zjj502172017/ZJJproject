// use std::io;
// use std::cmp::Ordering;
use std::io::stdin;

fn main() {
    // loop {
    let mut s = String::new();

    stdin().read_line(&mut s).unwrap();
    println!("s.len()-1={0}", &s[..s.len() - 2]);
    match &s[..s.len() - 1] {
        "quit" => println!("break msg"),
        "help" => println!("help msg"),
        "list" => println!("list msg"),
        _ => println!("????"),
    }
    // }
    // println!("Hello, world!");
}
