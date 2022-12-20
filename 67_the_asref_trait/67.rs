use std::fmt::Display;

fn print_it<T>(input: T) 
where 
    T: Display + AsRef<str>
{
    println!("{input}");
}

fn main() {
    // print_it(9); >> error
    print_it("Please print me");
    print_it(String::from("Please print me")); 
}