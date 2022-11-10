fn main() {
    print!("This\nis\na\nbunch\nof\nlines");
    print!(r#"C:\dev\rust"#); //경로가 깔끔하게 나온다.
    println!("
DEBUG
내용");
    let variable = 9000;
    let ptr = &variable;
    println!("{}", variable);
    println!("{:?}", variable); // debug 출력
    println!("{:X}", variable); //hax로 출력
    println!("{:p}", ptr); //포인터 주소값 출력
    println!("{:b}", variable); //바이트 단위로 출력

    let title = "TODAY'S NEWS";
    println!("{:-^30}", title);
    let bar = "|";
    println!("{: <15}{: >15}", bar, bar);
    let a = "SEOUL";
    let b = "TOKYO";
    println!("{city1:-<15}{city2:->15}", 
        city1 = a,
        city2 = b
    );
}