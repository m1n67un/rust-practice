use std::thread::available_parallelism;

fn main() {
    let (mut x, mut y) = (9, 10);
    (x, y) = (10, 11);

    let a = available_parallelism().unwrap();
    println!("We have a {a} cores")
    
    let my_num = "99".parse::<u32>().map(|num| &num);
    println!("{my_num:?}");
}