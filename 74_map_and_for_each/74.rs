fn main() {
    let num_vec = vev![2, 4, 6];
    
    let double_vec = num_vec
        .iter()
        .map(|number| number * 2)
        .collect::<Vec<_>>();

    println!("{double_vec:?");

    num_vec
        .iter()
        .enumerate()
        .for_each(|(index, number)| {
            println!("The number at index {index} is {number}");
        });
}

fn main() {
    let num_vec = vec![2, 4, 6];

    let new_thing = num_vec
        .iter()
        .enumerate()
        .map(|(index, number)| {
            println!("The number at index {index} is {number}");
            0
        })
        .collect::<Vec<_>>();

    println!("{new_thing");
}