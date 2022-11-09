fn main() {
    println!("Hello, world!");

    // casting = simple type change
    let my_number: u16 = 8;
    let second_number: u8 = 10;
    let third_number = my_number + second_number as u16; // 기존 second_number의 u8 타입이 u16으로 변환됨. second_number as u16

    let other_number = 'a' as u8; //ASCII 문자가 아닌 경우 오류
    println!("Hello, world! My number is {}", other_number);
}