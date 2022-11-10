fn main() {
    let num = 5;
    let second_num = 10;

    if num == 7 && second_num == 10 {
        println!("They both match");
    } else if num == 6 { // && and || or
        println!("It's six");
    } else {
        println!("It's a different number");
    }

    /*: 아래와 같이 코드를 작성하는 경우 if 조건문에 ()로 감싸져있는데 오류는 나지 않고, warning이 뜬다. 굳이 쓰지 않아도 된다는 말이다.
        if(num == 7) {
            println!("It's seven");
        }
    */
}