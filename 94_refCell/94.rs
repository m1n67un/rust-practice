/*
use std::cell::Cell;

fn main() {
    let my_cell = Cell::new(String::from("I am a String"));
    my_cell.set(String::from("I am a String??!?!?!?"));
    let my_string = my_cell.get();
}
*/

use std::cell::Cell;

#[derive(Debug)]
struct User {
    id: u32,
    year_registered: u32,
    username: String,
    active: RefCell<bool>
}

fn main() {
    let user_1 = User {
        id: 1,
        year_registered: 2020,
        username: "User 1".to_string(),
        active: RefCell::new(true)
    };

    println!("{user_1:?}");
    let first_reference = user_1.active.borrow_mut();

    println!("{user_1:?}");
    *first_reference = false;

    println!("{user_1:?}");
    drop(first_reference);

    println!("{user_1:?}");
}