struct Person {
    name: String,
    real_name: String,
    height: u8,
    happiness: bool
}

struct Person2 {
    name: String,
    height: u8
}

impl Person2 {
    fn from_person(input: Person) -> Self {
        let Person { name, height, ..} = input;

        Self {
            name,
            height
        }
    }
}

fn main() {
    let papa_doc = Person {
        name: "Papa Doc".to_string(),
        real_name: "Clarence".to_string(),
        height: 170,
        happiness: false
    };

    let person2 = Person2::from_person(papa_doc);

    let Person { 
        name: a,
        real_name: b,
        height: c,
        happiness: d
    } = papa_doc;

    println!(
        "They call him {} but his real name is {}. He is {} cm tall and is he happy? {}",
        a, b, c, d
    );
}