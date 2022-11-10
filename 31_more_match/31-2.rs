fn match_num(input: i32) {
    match input {
        number => println!("It's the number {}", number),
        number @ 0..=10 => println!("It's between 0 and 10, It's the number {}", number),
        _ => println!("It's greater than ten")
    }
}

fn main() {
    match_num(10);
    match_num(100);
}