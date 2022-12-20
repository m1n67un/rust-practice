struct Animal {
    name: String
}

trait Canine {
    fn bark(&self);
    fn run(&self);
    fn attck(&self) {
        println!("Woof Woof!");
    }
    fn go(&self) {
        println!("I am running!");
    }
}

impl Canine for Animal {
    fn attack(&self) {
        println!("멍멍! 나는 {}라고 한다", self.name);
    }
}

fn main() {
    let my_animal = Animal {
        name: "Mr. Mantle".to_string();
    }

    my_animal.attack();
    my_animal.go();
}