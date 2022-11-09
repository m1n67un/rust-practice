/*
fn number() -> () {
    8;
}
*/

fn number() -> i32 {
    8
}

fn main() {
    let number = number();

    // :? Debug print
    println!("{:?}", number);
}