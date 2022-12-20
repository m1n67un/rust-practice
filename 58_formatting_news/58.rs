#[derive(Debug)]
struct Book {
    title: Stirng,
    year: u16
}

fn main() {
    let my_book = Book {
        title: "Some title".to_string(),
        year: 1919
    };
    let book_2 = Book {
        title: "Book 2".to_string(),
        year: 2020
    };

    println!("Got books:\n{:?}\n{:?}", my_book, book_2);
    println!("Got books:\n{my_book:?}\n{book_2:?}");
    println!("My book name: {my_book:*^16?}");

    let width = 16;
    println!("My book name: {my_book:*^width$?}");
}