fn main() {
    let mut num = 10;
    let num_change = &mut num;
    *num_change += 10;
    let num_ref = &num;
    println!("{}", num_ref);

    let country = "대한민국";
    let country_ref = &country;
    let country  = 8;
    println!("{}, {}", country_ref, country);
}