/*
fn main() {
    let num_vec = vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100];

    println!("{:?}", num_vec.iter().find(|&n| n & == 0));
    println!("{:?}", num_vec.iter().find(|&number| number * 2 == 30));

    println!("{:?}", num_vec.iter().position(|&num| num % 3 == 0));
    println!("{:?}", num_Vec.iter().position(|&num| num * 2 == 30));
}
*/

fn main() {
    let even_odd = vec!["even", "odd"];

    let six_items = even_odd.into_iter().cycle().take(6).collect::<Vec<_>>();
    println!("{even_odd_vec:?}");
}