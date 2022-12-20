fn main() {
    /*
    let my_number = 9;
    let anoymous_function = || println("I am a function");
    let closure = || println!("{my_number}");

    .iter().map().filter().inspect().collect()
    */
    let my_vec = vec![8, 9, 10, 9];

    let fourth = my_vec.get(3).unwrap_or_else(|| {
        if my_vec.get(0).is_some() {
            &my_vec[0]
        } else {
            &0
        }
    });

    println!("{fourth}");
}