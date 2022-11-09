fn main() {
    let my_number = 9_u8;
    let other_number = 10_000_000_000u64; // _의 경우 무시한다.
    
    let my_number = 9.; //f64
    let other_number = 9; //i32
    //f32 / f64 // 8 bytes
    println!("{}", my_number as i32 + other_number);
    println!("{}", my_number+ other_number as f64);
}