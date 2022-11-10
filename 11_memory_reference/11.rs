fn main() { //&, C의 포인터의 개념과 같다.
    let num = 15;
    let single_reference = &num; // num 값을 가져온다.
    let double_reference = &single_reference; // 가능
    let result = &&&&&double_reference; //문제 없다.
}