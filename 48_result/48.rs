fn check_error(input: i32) -> Result<(), ()> {
    if input % 2 == 0 {
        Ok(());
    } else {
        Err(());
    }
}

// enum Option<T> {
//     None,
//     Some(T),
// }

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

fn main() {
    //let variable = check_error();
    /*
    if check_error(5).is_ok() {
        println!("it's okay, guys!");
    } else {
        println!("It's an error, guys!");
    }
    */

    /*
    match check_error(5) {
        Ok(_) => println!("Okay guys"),
        Err(_) => println!("It's an error")
    }
    */
    // None.unwrap -> panic
    // Err.unwrap -> panic

    check_error(5).unwrap();
}