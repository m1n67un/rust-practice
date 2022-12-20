use std::fmt::Debug;

#[derive(Debug)]
struct MyStruct {
    number: usize
}

fn print_as_debug<T>(input: T) 
where 
    T: Debug
{
    println!("{input:?");
}

fn main() {}

///////////////////////////////////////

use std::fmt::Debug;

struct ThingsToAdd {
    first_thing: u32,
    second_thing: f32
}

fn main() {
    let my_thing = ThingsToAdd {
        first_thing: 32,
        second_thing: 8.8
    }

    let second_thing = ThingsToAdd {
        first_thing: 32,
        second_thing: 8.8
    }

    let sum = my_thing + second_thing;
}