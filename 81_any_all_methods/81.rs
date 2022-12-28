fn in_char_vec(char_vec: &Vec<char>, check: char) {
    println!("Is {check} inside? {}", char_vec.iter().any(|&character| character == check));
}

fn main() {
    let char_vec = ('a'..'韓').collect::<Vec<char>>();
    in_char_vec(&char_vec, "i");
    in_char_vec(&char_vec, "뷁");
    in_char_vec(&char_vec, "姜");

    let smailer_vec = ('A'..'z').collect::<Vec<char>>();
    println!("All alphabetic? {}", smailer_vec.iter().all(|&character| character.is_alphabetic()));
    println!("All less than character 행? {}", smailer_vec.iter().all(|&c| c < '행'));
    println!("{smailer_vec:?}");
}