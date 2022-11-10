// 오류
fn return_it() -> &'static String {
    let country = String::from("대한민국");
    &country // return &String
}

fn main() {
    return_it();
}