struct SomeStruct {
    name: String,
    number: u8,
    data: Box<[u8; 10000]>
}

fn main() {
    let my_struct = SomeStruct {
        name: "Hi there".to_string(),
        number: 0,
        data: Box::new([9; 1000])
    };

    let my_box = Box::new(9);

    println!("{my_box}");
    println!("{}", *my_box);
    println!("The struct is {} bytes", std::mem::size_of_val(&my_struct));
}