// tuple - rust의 컬렉션 타입 중 하나
// Vec<String>
// tuple 같은 타입이 아니여도 된다.

// Vec은 같인 타입만
// Vec<(String, i32)>

fn main() {
    let tuple = (8, "Dave MacLeod", vec![8, 9, 10]);
    println!("{:?}", tuple);

    let random = ("Here is a name", 8 , vec!['a'], 'b', [8, 9, 10], 7.7);
    println!(
        "Inside the tuple is: First item: {:?}
Second item: {}
Third item: {:?}
Fourth item: {:?}
Fifth item: {:?}
Sixth itrem; {}",
        random.0,
        random.1,
        random.2,
        random.3,
        random.4,
        random.5,
    );

    //let vec![("Hey", 9), ("Hello there", 8972983)];

    let str_vec = vec!["one", "two", "three"];
    let (a1, b1, c1) = str_vec;
    println!("Item a is: {}", a1);

    let str_tuple = ("one", "two", "three");
    let (a2, b2, c2) = str_tuple;
    // let (a2, _, _) = str_tuple; => _로 warning을 출력되지 않게 할 수 있다.
    println!("item a is: {}", a2);

    let str_array = ["one", "two", "three"];
    let [a3, b3, c3] = str_array;
    println!("item a is: {}", a3);


}