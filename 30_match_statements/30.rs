fn main() {
    let sky = "cloudy";
    let temperature = "warm";

    match (sky, temperature) {
        ("cloudy", "cold") => println!("It's not very nice today"),
        ("clear", "warm") => println!("It's a nice day"),
        ("cloudy", _) => println!("Cloudy and something else"),
        _ => println!("Not sure what the weather is."),
    }

    let children = 5;
    let married = true;
    match (children, married) {
        (children, married) if married == false => println!("Not married with {} children", children),
        (c, m) if c == 0 && m/* == true */ => println!("Married but with no children"),
        _ => println!("Some other type of marriage and children combination"):
    }
}