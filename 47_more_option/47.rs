// Option<T>

fn take_fifth(value: Vec<i32>) -> Option<i32> {
    if value.len() < 5 {
        None
    } else {
        Some(value[4]) // i32
    }
}

fn main() {
    let new_vec = vec![1, 2, 2];
    let index = take_fifth(new_vec); //: Option<i32>
    
    /*: println!("{}", index.unwrap());
    match index {
        Some(number) => println!("I got a number: {}", number),
        None => println!("There was nothing inside"),
    }
    */

    index.expect("Needed at least five items - make sure Vec has at least five");
}