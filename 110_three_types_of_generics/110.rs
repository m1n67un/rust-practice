use std::fmt::Display;

fn print<T: Display>(input: T) {
    println!("Hi, I'm a {input}");
}

fn print_2(input: impl Display) {
    println!("Hi, I'm a {input}");
}

fn print_3(input: Box<dyn Display>) {
    println!("Hi, I'm a {input}");
}

fn main() {
    print_2(8);
    print_3(Box::new(String::from("I am a String")));
}