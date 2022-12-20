#[derive(Debug)]
struct Library {
    library_type: LibraryType,
    books: Vec<String>
}

#[derive(Debug)]
enum LibraryType {
    City,
    Country
}

impl Library {
    fn add_book(&mut self, book: &str) {
        self.books.push(book.to_string());
    }

    fn new() -> Self {
        Self {
            library_type: LibraryType::City,
            books: Vec::new()
        }
    }
}

impl Iterator for Library {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        match self.books.pop() {
            Some(book_title) => Some(book_title + "is found!"), // String + &str
            None => None
        }
    }
}

fn main() {
    let mut my_library = Library::new();
    my_library.add_book("The Doom of the Darksword");
    my_library.add_book("Demian - die Geschichte einer Jugend");
    my_library.add_book("구운몽");
    my_library.add_book("天下");
    println!("{:?}", my_library.books);

    for item in my_library {
        println!("{item}");
    }
}