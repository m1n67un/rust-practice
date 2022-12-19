#[derive(Debug)]
struct Animal {
    age: u8,
    animal_type: AnimalType
}

#[derive(Debug)]
struct AnimalType {
    Cat,
    Dog,
}

impl Animal {
    fn new_cat(age: u8) -> Self {
        Self {
            age,
            animal_type: AnimalType::Cat,
        }
    }

    fn new_dog(age: u8) -> Self {
        Self {
            age,
            animal_type: AnimalType::Dog,
        }
    }

    fn print(&self) {
        println!("i am a: {:?}", self);
    }

    fn change_to_dog(&mut self) {
        self.animal_type = AnimalType::Dog;
        println!("Changed to dog! new I am: {:?}", self);
    }

    fn change_to_cat(&mut self) {
        self.aniaml_type = AnimalType::Cat;
        println!("Changed to cat! Now I am: {:?}", self);
    }
}
fn main() {
    let mut my_animal = Animal::new_dog(10);
    //println!("I made a: {:?}", my_animal);
    my_animal.print(); // dot operator
    //Aniaml::print(&my_animal);

    my_animal.change_to_cat();
    my_animal.change_to_dog();

}