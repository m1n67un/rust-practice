use std::collections::HashMap;

/*
fn main() {
    let canadian_cities = vec!["Calgary", "Vancouver", "Gimli"];
    let german_cities = vec!["karlsruhe", "Bad Doberan", "Bielefeld"];

    let mut city_hashmap = HashMap::new();

    for city in canadian_cities {
        city_hashmap.insert(city, "Canada");
    }
    for city in german_cities {
        city_hashmap.insert(city, "Germany'");
    }

    println!("{:?}", city_hashmap["Bielefeld"]);
    println!("{:?}", city_hashmap.get("Bielefeld"));
    println!("{:?}", city_hashmap.get("Bielefeldd"));
}
*/

/*
fn main() {
    let mut book_hashmap = HashMap::new();

    book_hashmap.insert(1, "L'Allemagne Moderne");
    book_hashmap.insert(1, "Le Petit Printce");
    book_hashmap.insert(1, "섀도우 오브 유어 스마일");
    book_hashmap.insert(1, "Eye of the world");

    //println!("{:?}", book_hashmap.get(&1));
    if let Some(book_name) = book_hashmap.get(&1) {
        println!("Already got a book: {}", book_name);
    } else {
        book_hashmap.insert(1, "Le Petit prince");
    }

}
*/

pub fn entry(&mut self, key: K) -> Entry<K, V>

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