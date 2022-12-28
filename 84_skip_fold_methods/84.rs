/*
fn main() {
    let ten_chars = ('a'..).take(10).collect::<Vec<_>>();
    println!("{ten_chars:?}");

    let skip_then_ten_chars = ('a'..).skip(1300).take(10).collect::<Vec<_>>();
    println!("{skip_then_ten_chars:?}");
}
*/

fn main() {
    let some_numbers = vec![9, 6, 9, 10, 11];

    println!("{}", some_numbers.iter().fold(0, |total_so_far, next_number| total_so_far + next_number));
}