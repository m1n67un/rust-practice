use std::fmt::Display;

fn generic_function<T: Display>(input: T) {
    println!("{input}");
}

type MyString = impl Display;

fn impl_function(input: impl Display) {
    println!("{input}");
}
fn returns_a_closure() -> impl Fn(i32) {
    |x| println!("{x}")
}

fn main() {
    let my_number = 9;
    let my_closure = returns_a_closure();
    my_closure(my_number);

    generic_function(8);
}