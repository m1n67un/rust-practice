fn double(input: i32) -> i32 {
    input * 2
}

fn triple(input: i32) -> i32 {
    input * 3
}

fn main() {
    /*: 아래 코드는 오류, 수정 불가능
        let num = 10;
        num = 9;
    */
    let mut num = 10;
    num = 9;

    let num = 10;
    let num = "My variable"
    println!("{}", num);

    let x = 0;
    let x = double(x);
    let x = triple(x);

    println!("{}", x);

    let variable = 9;
    println!("{}", variable);
    { //: Code Block
        let variable = "Some string";
        println!("{}", variable);
    }
    println!("{}", variable);
}