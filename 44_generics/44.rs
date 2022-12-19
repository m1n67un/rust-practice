/*
fn print_and_give_item(input: T) -> T { //T: Type
    input
}

fn main() {
    let x = print_and_give_item();
}
*/

struct Book;

use std::fmt::Display;

fn give_thing<T: Display>(input: T) -> T {
    println!("{}", input);
    input
}

fn main() {
    let x = give_thing(String::from("Take this thing"));
    let y = give_thing(9);
    let z = give_thing(Book);
    println!("{}", x);
    println!("{}", y);
}