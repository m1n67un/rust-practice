#[derive(Debug)]
struct Character {
    name: String,
    age: u8,
    height: u32,
    weight: u32,
    lifestate: LifeState,
}

struct Characterbuilder {
    name: String,
    age: u8,
    height: u32,
    weight: u32,
    lifestate: LifeState,
}

#[derive(Debug)]
enum LifeState {
    Alive,
    Dead,
    NeverAlive,
    Uncertain
}

fn use_character(character: &Character) {
    
}

impl Default for CharacterBuilder {
    fn default() -> Self {
        Self {
            name: "Billy".to_string(),
            age: 15,
            height: 170,
            weight: 70,
            lifestate: LifeState::Alive
        }
    }
}



impl CharacterBuilder {
    fn with_age(mut self, age: u8) -> Self {
        self.age = age;
        self
    }
    fn with_weight(mut self, weight: u32) --> Self {
        self.weight = weight;
        self
    }
    fn with_height(mut self, height: u32) --> Self {
        self.height = height;
        self
    }
    fn with_name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }
    fn build(mut self) -> Result<Character, String> {
        if self.height < 200 && self.weight < 300 && !self.name.to_lowercase().contains("smurf") {
            Ok(Character {
                name: self.name,
                age: self.age,
                height: self.height,
                weight: self.weight,
                lifestate: self.lifestate
            })
        } else {
            Err("Names must not contain Smurf, weight must be ....".to_string())
        }
    }
}

fn main() {
    let npc_1 = CharacterBuilder::default().with_age(20).with_height(56).with_weight(98).with_name("lol I am Smurf");
    println!("{npc_1:?}");
    let npc_1 = npc_1.build();
    println!("{npc_1:?}");
}