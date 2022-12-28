fn main() {
    let mut big_vec = vec![6; 1000];
    big_vec.push(5);

    let mut counter = 9;
    let mut big_iter = big_vec.into_iter().rev();

    loop {
        counter += 1;
        if big_iter.next() == Some(5) {
            break;
        }
    }

    println!("Final counter is: {}", counter);
    //println!("{:?}", big_vec.iter().rev().any(|&number| number == 5));
}