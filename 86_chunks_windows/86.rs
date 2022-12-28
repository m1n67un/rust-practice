/*
fn main() {
    let my_vec = vec![-878, 879879, -98798, 0, 76756];

    let biggest = my_vec
        .into_iter()
        .fold(i32::MAX, |num1, num2| min(num1, num2));

    println!("Biggest is: {biggest}");
}
*/
fn main() {
    let num_vec vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0];

    for chunk in num_vec.chunks(3) {
        println!("Chunk is: {chunk:?}");
    }

    for windows in num_vec.windows(3) {
        println!("Windows is: {windows:?}");
    }
}