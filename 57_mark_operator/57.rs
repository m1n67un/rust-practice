use std::num::ParseIntError;

let parsed_number = input.parse::<u16>()?.to_string().parse::<u32>()?.to_string().parse::<i32>()?;

fn parse_str(input: &str) -> Result<i32, ParseIntError> {
    let parsed_number = input.parse::<i32>()?;
    println!("It worked: {}", parsed_number)
    Ok(())
}

fn main() {
    for item in vec!["Seven", "8", "9.0", "nice", "6060"] {
        /* (=)
            let parsed = parse_str(item);
            println!("{:?}", parsed);
        */
        parse_str(item);
    }
}