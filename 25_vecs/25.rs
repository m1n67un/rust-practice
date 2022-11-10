// Vec<String>
// Vec<u8>

fn main() {
    let name1 = String::from("Windy");
    let name2 = String::from("Gomesy");

    let mut vec = Vec::new();
    println!("Space for vec: {}", vec.capacity());
    vec.push(name1);
    println!("Space for vec: {}", vec.capacity());
    vec.push(name2);
    println!("Space for vec: {}", vec.capacity());

    /*: push없이 아래와 같이 표현할 수 있다.
        let vec = vec![name1, name2];
    */

    println!("My cats are {:?}", vec);


}