/*
fn add_is_great(country_name: &mut String) {
    country_name.push_str(" is great!");
    println!("Now it says: {}", country_name);
}

fn main() {
    let mut country = "캐나다".to_string();
    add_is_great(&mut country);
    add_is_great(&mut country);
}
*/

fn add_is_great(mut country_name: String) -> String {
    country_name.push_str(" is great!");
    println!("Now it says: {}", country_name);
    country_name
}

fn main() {
    let mut country = "대한민국".to_string();
    add_is_great(country);
}