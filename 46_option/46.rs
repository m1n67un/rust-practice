// Option<T>

fn take_fifth(value: Vec<i32>) -> Option<i32> {
    if value.len() < 5 {
        None
    } else {
        Some(value[4]) // i32
    }
}

fn main() {
    let new_vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 10];
    let index = take_fifth(new_vec);
    println!("{:?}", index);
}