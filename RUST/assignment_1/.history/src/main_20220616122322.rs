use std::io::stdin;
use forward_goto::rewrite_forward_goto;

#[rewrite_forward_goto]
fn decode_msg( is_alan_turing: bool) {
    if is_alan_turing {
        forward_goto!('turing_complete);
    }

    println!("You'll need a lot of luck for this");
    let  luck = true;
    if luck {
        println!("Seems you were lucky");

        forward_label!('turing_complete);

        println!("Message decoded!");
    } else {
        println!("No luck today...");
    }
} 

fn main() {
    println!("This is a menu, you can enter:");
    println!("1.help");
    println!("2.test");
    println!("3.quit");
    decode_msg(true);
    decode_msg(false);
    // loop {
    //     println!("please enter a message:");
    //     let mut s = String::new();
    //     stdin().read_line(&mut s).unwrap();
    //     match &s[..s.len() - 2] {
    //         "quit" => break,
    //         "help" => println!("help message"),
    //         "test" => println!("test message"),
    //         _ => println!("unknow command"),
    //     }
    // }

}
