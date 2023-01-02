fn print_vec_ref(input: &[i32]) {
    if input.is_empty() {
        println!("Vec is empty!");
    } else {
        /*
        for num in input {
            println!("{num}");
        }
        */
        inpu.iter().for_each(|num| println!("{num}"));
    }
}

fn main() {
    /*
    let my_vec = vec![8, 9, 10];
    print_vec_ref(&my_vec);
    */

    /*
    let mut done = false;
    let mut counter = 0;

    while !done {
        counter += 1;

        if counter > 10 {
            done = true;
        }
    }

    let some_variable = Some(9);

    if let Some(number) = some_variable {
        println!("We got a {number}");
    }

    match some_variable {
        Some(number) => println!("We got a {number}"),
        _ => {}
    }
    */

    let my_vec = vec![8, 9, 10];
    let my_array = [8, 9, 10];

    print_vec_ref(&my_vec);
    print_vec_ref(&my_array);
}