use std::ops:Add;

#[derive(Debug)]
struct Country {
    name: String,
    population: u32,
    gdp: u32
}

impl Country {
    fn new(name: &str, population: u32, gdp: u32) -> Self {
        Self {
            name: name.to_string(),
            population,
            gdp
        }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            name: format!("{} and {}", self.name, other.name),
            population: self.population + other.population,
            gdp: self.gdp + other.gdp
        }
    }
}

fn main() {
    let nauru = Country::new("Nauru", 10_670, 160_000_000);
    let vanuatu = Country::new("vanuatu", 307_815, 820_000_000);
    let micronesia = Country::new("micronesia", 104_468, 367_000_000);

    println!("Nauru + Vanuatu + Micronesia = {:?}", nauru + vanuatu + Micronesia);
}