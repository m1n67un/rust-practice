fn print_country(country_name: &String) {
    println!("my country is {}", country_name);
}

fn main() {
    let mut country = "대한민국".to_string();
    print_country(&country);
    print_country(&country);
    print_country(&country);
}