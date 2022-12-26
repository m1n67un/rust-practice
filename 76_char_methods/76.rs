fn main() {
    let big_string = "Hello there, I am a &str";

    // big_string.chars().for_each(|c| println!("{c}"));
    // println!("big_string has {} characters", big_string.chars().count());

    // big_string.char_indices().for_each(|(index, charrrrr)| {
    //     println!("At index {index} is the char {charrrrr}");
    // });

    let my_vec = vec![8, 9, 10];
    my_vec.iter().for_each(|_| println!("We don't care about the number"));
}