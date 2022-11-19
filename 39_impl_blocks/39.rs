// impl blocks


#[derive(Debug)] //: Debug Print
struct Animal {
    age: u8,
    animal_type: AnimalType
}

#[derive(Debug)] //: Debug Print
enum AnimalType {
    Cat,
    Dog
}

impl Animal {
    fn new_old_cat() -> Self {
        Self {
            age: 15,
            animal_type: AnimalType::Cat,
        }
    }
}

impl Animal {
    fn new_cat(age: u8) -> Self { //: Self = Animal __ function signature
        Self {
            age,
            animal_type: AnimalType::Cat
        }
    }

    fn new_dog(age: u8) -> Self {
        Self {
            age,
            animal_type: AnimalType::Dog,
        }
    }

    fn print(&self) {
        println!("I am a: {:?}", self);
    }

    fn change_to_dog(&mut self) {
        self.animal_type = AnimalType::Dog;
        println!("Changed to dog: Now I am: {:?}", self);
    }

    fn change_to_cat(&mut self) {
        self.animal_type = AnimalType::Cat;
        println!("Changed to cat: Now I am: {:?}", self);
    }
}

fn main() {
    let my_vec = vec![7, 8];
    println!("The Vec length is: {}", my_vec.len()); //method

    let my_cat = Animal::new_cat(10);
    println!("I mad a: {:?}", my_cat);

    let mut my_animal = Animal::new_dog(10);
    my_animal.print(); // syntactic sugar
    my_animal.change_to_cat();
    my_animal.change_to_dog();

    let my_old_cat = Animal::new_old_cat();
}