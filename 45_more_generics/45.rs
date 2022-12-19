use std::fmt::Display;
use std::fmt::PartialOrd;

fn compare_and_print<T, U>(
    statement: T, 
    num_1: U, 
    num_2: U
) where 
        T: Display,
        U: Display + PartialOrd, 
{
    println!(
        "{}! Is {} greater than {}? {}",
        statement,
        num_1,
        num_2,
        num_1 > num_2
    );
}

fn main(c) {
    compare_and_display("Listen up!", 9, 8);
}