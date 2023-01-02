/*
enum List {
    Content(i32, Box<List>),
    NoContent
}

fn main() {
    let my_list = List::Content(8786, Box::new(List::NoContent))
}
*/
trait Booky {}

struct Book {
    name: String
}
struct BigBook;

struct City {
    year_founded: i32
}

impl Booky for Book {}
impl Booky for BigBook {}
impl Booky for City {}

fn main() {
    let my_city = City {
        year_founded: 1989
    };

    let vec_of_booky_things: Vec<Box<dyn Booky>> = vec![
        Box::new(Book {
            name: "my book".to_string(),
        }),
        Box::new(BigBook),
        Box::new(my_city)
    ];
}