fn main() {
    //1
    let mut new_vec = Vec::new();
    let mut counter = 1;
    while counter < 11 {
        new_vec.push(counter);
        counter += 1;
    }
    println!("{new_vec:?}");

    //2
    let new_vec = (1..=10).collect::<Vec<_>>();
    println!("{new_vec:?}");

    //3
    let my_vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let new_vec = my_vec
        .into_iter()
        .skip(1)
        .skip(1)
        .skip(3)
        .take(4)
        .collect::<Vec<i32>>();
    println!("{new_vec:?}");
}