use std::mem::size_of;

fn main() {
    println!("Size of a char: {}", std::mem::size_of::<char>()); // string 내부에 {} 기입한 경우, 뒤 매개변수에 값을 넣지 않으면 오류 호출
    println!("Size of a char: {}", size_of::<char>());

    println!("Size of string containing 'a': {}", "a".len()); // 글자수가 아닌 바이트수
    println!("Size of string containing '&': {}", "&".len());
    println!("Size of string containing '國': {}", "國".len()); // 한자는 바이트 3바이트, 이집트어의 경우 4바이트

    let slice = "Hello!";
    println!("Slice is {} bytes and also {} characters.", slice.len(), slice.chars().count());
    let slice2 = "안녕!";
    println!("Slice is {} bytes but only {} characters.", slice2.len(), slice2.chars().count());
}