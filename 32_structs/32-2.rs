struct FileDirectory;

#[derive(Debug)] // attribute
struct Color(u8, u8, u8);

// name struct
#[derive(Debug)] // attribute
struct Country {
    population: u32,
    capital: String,
    leader_name: String
}

fn main() {
    let myColor = Color(20, 50, 100);
    println!("The second color is {:?}", myColor);

    let canada = Country {
        population: 35_000_000,
        capital: "Ottawa".to_string(),
        leader_name: "Justin Trudeau".to_string()
    };

    // u32
    println!("The population: {}", canada.population);

    println!("The country is: {:#?}", canada);
}