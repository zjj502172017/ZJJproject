use forward_goto::rewrite_forward_goto;

#[rewrite_forward_goto]
fn decode_msg(luck: impl Fn() -> bool, is_alan_turing: bool) {
    if is_alan_turing {
        forward_goto!('turing_complete);
    }

    println!("You'll need a lot of luck for this");
    
    if luck() {
        println!("Seems you were lucky");

        forward_label!('turing_complete);

        println!("Message decoded!");
    } else {
        println!("No luck today...");
    }
}