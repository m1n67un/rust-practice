fn prints_number(number: i32) {
    println!("{}", number);
}

fn prints_string(input: String) {
    println!("{}", input);
}

// copy - copy types
// clone - non-copy types
fn main() {
    let num = 8;
    prints_number(num);
    prints_number(num);

    let country = "Austria".to_string();
    prints_string(country.clone());
    prints_string(country);
    
    //: 위에서 한번 쓰였기 떄문에 소유권이 위 매개변수에 담겨져있기 때문에, 아래코드는 오류가 호출한다.
    prints_string(country);
}