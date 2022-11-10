fn main() {
    let arr = ["one", "two"]; // [&str: 2]
    let arr2 = ["one", "two"]; // [&str: 2]

    println!("Is array the same as arr2? {}", arr == arr2);

    //buffer
    let arr = [0; 640];
    println!("{:?}", arr);

    let arr = ["1월", "2월"];
    println!("{}", arr[0]);
    println!("{:?}", arr.get(0));
    // Some None Option enum
}