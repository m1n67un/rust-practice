use std::fmt::{Debug, Display};

trait Prints: Debug {
    fn debug_print(&self) where Self: Debug{
        //println!("I like to print things");
        println!("I am: {:?}", self);
    }
    fn display_print(&self) where Self: Display{
        println!("I am: {}", self);
    }
}

#[derive(Debug)]
struct Person;
#[derive(Debug)]
struct Building;

impl<T> Prints for T {

}

fn main() {
    let my_person = Person;
    let my_building = Building;
    my_person.display_print();
    let my_string = String::from("Hello there");
    my_string.debug_print();
    my_string.display_print();
}