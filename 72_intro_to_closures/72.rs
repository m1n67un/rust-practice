.iter().map(|item| {
    let my_number = 7;
    item + my_number
    })
    .collect()

fn main() {
    /*
    let my_number = 10;
    let my_closure = |x: i32| println!("{}", x + my_number);

    my_closure(9);
    */

    let my_closure = || {
        let my_number = ?;
        let other_number = 10;
        println!("The two numbers are {my_number} and {other_number}");
        my_number + other_number
    };

    let my_var = my_closure();
    println!("{my_var}");
}