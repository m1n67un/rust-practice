/*
fn returns_reference() -> &'static str {
    //let my_string = "David".to_string();
    "David"
}
*/

struct Book<'a> {
    name: &'a str
}
use std::fmt::Display;

fn print_thing<T: Display>(input: T) {

}

fn main() {
    let my_book = Book {
        name: "my book"
    }
}