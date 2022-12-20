use std::collections::HashMap;

fn main() {
    let some_numbers = vec![0, 1, 2, 3, 4, 5];
    let some_words = vec!["zero", "one", "two", "three", "four", "five"];

    let number_word_hashmap: HashMap<i32, &str> = some_numbers
        .into_iter()
        .zip(some_words.into_iter())
        .collect();
        //.collect::<HashMap<_, _>>();
    
    number_word_hashmap.iter().for_each(|stuff| {
        println!("{stuff:?}");
    })
}