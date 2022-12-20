/*
use std::fmt:Display;

fn print_vec<T: Display>(input: &Vec<T>) {
    for item in input {
        print!("{item} ");
    }
    println!();
}

fn main() {
    let array_vec = String::from("thth");
    print_vec(&array_vec);

    let str_vec = Vec::from("What kind of vec is this?");
    print_vec(&str_vec);

    let string_vec = Vec::from("What kind of vec is a String vec?");
    print_vec(&string_vec);
}
*/

#[derive(Debug)]
struct City {
    name: String.
    population: u32
}

impl City {
    fn new(name: &str, population: u32) -> Self {
        Self {
            name: name.to_string(),
            population
        }
    }
}

//Country::from(vec![City, city])
impl Form<Vec<City>> for Country {
    fn from(cities: Vec<City>) -> Self {
        Self {
            cities
        }
    }
}

impl Country {
    fn print_cities(&self) {
        for city in &self.cities {
            println!("{:?} has a population of {:?}", city.name, city.population);
        }
    }
}

fn main() {
    let helsinki = City::new("Helsinki", 631_695);
    let turku = City::new("Turku", 186_756);

    let finland_cities = vec![helsinki, turku];
    //: let finland = Country::from(finland_cities); >> 아래와 같음 (=)
    let finland: Country = finland_cities.into();
    finland::print_cities();
}