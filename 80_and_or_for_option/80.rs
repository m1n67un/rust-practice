/*
fn main() {
    let one = false;
    let two = false;
    let three = false;
    let four = true;

    println!("{}", one || three);
    println!("{}", one || two || three || four);
}
*/

fn main() {
    let first_try = vec![Some("success!"), None, Some("success!"), Some("success!"), None];
    let second_try = vec![None, Some("success!"), Some("success!"), Some("success!"), Some("success!")];
    let third_try = vec![Some("success!"), Some("success!"), Some("success!"), Some("success!"), None];

    for index in 0..first_try.len() {
        println!("{:?}", first_try[index].or(second_try[index]).or(third_try[index]));
        // and, and => 해당 index에 None이 존재하면 None 출력, 없으면 and의 맨 마지막 값 출력.
    }
}