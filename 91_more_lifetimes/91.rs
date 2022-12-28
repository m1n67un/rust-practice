// 'static
/*
struct Book<'a> {
    name: &'a str,
    second_name: &'a str
}
use std::fmt::Display;

fn print_thing<T: Display>(input: T) {

}

fn main() {
    let my_book_title = "my_book_title".to_string();

    // let my_book = Book {
    //     name: "my book"
    // }
}
*/

struct Adventurer<'a> {
    name: &'a str,
    hit_points: u32
}

impl Adventurer<'_> {
    fn take_damage(&mut self) {
        self.hit_points -= 20;
        println!("{} has {} hit points left!", self.name, self.hit_points);
    }
}

fn main() {

}