pub fn entry(&mut self, key: K) -> Entry<K, V>

use std::collections::HashMap;

fn main() {
    let book_collection = vec![
        "L'Allemagne Moderne",
        "Le Petit Prince",
        "섀도우 오브 유어 스마일",
        "Eye of the World",
        "Eye of the World"
    ];

    //: .insert()
    //: .entry()

    let mut book_hashmap = HashMap::new();

    for book in book_collection {
        let number_of_books = book_hashmap.entry(book).or_insert(0);
        *number_of_books += 1;
        //book_hashmap.entry(book).or_insert(true);
    }

    /*
    for (book, true_or_false) in book_hashmap {
        println!("Do we have {}? {}", book, true_or_false);
    }
    */

    for (book, number) in book_hashmap {
        println!("{}: {} copies", book, number);
    }
}