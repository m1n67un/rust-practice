fn main() {
    // match
    let num: u8 = 5;

    match num { //switch
        0 => println!("It's a zero");
        1 => println!("It's a one");
        _ => println!("It's a different number"); // _ "I don't care" "anything else"
    }

    let second_num = match num { // switch
        0 => 23,
        1 => 65,
        _ => 0
    };
    
    println!("The second number is: {}", second_num);
}