use str::fmt;

#[derive(Debug)]
struct Cat {
    name: String,
    age: u8
}

impl fmt::Display for Cat {
    fn fmt(&self, f: &mut fmt:Formatter<'_>) -> fmt::Result {
        let name = &self.name;
        let age = self.age;
        write(f, "My cat's name is {name} and it is {age} years old)")
    }
}

fn main() {
    let mr_mantle = Cat {
        name: "Reggle Mantle".to_string(),
        age: 4
    };

    prinln!("Mr. mantle is a {mr_mantle:?}");
}