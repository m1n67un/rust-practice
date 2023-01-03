use std::ops::Deref;

struct Character {
    name: String,
    strength: u8,
    dexterity: u8,
    health: u8,
    intelligence: u8,
    wisdom: u8,
    charm: u8,
    hit_points: u8,
    alignment: Alignment,
}

impl Character {
    fn new(
        name: String,
        strength: u8,
        dexterity: u8,
        health: u8,
        intelligence: u8,
        wisdom: u8,
        charm: u8,
        hit_points: u8,
        alignment: Alignment,
    ) -> Self {
        Self {
            name,
            strength,
            dexterity,
            health,
            intelligence,
            wisdom,
            charm,
            hit_points,
            alignment,
        }
    }
}

enum Alignment {
    Good,
    Neutral,
    Evil,
}

impl Deref for Character {
    type target = i8;

    fn deref(&self) -> &Self::Target {
        &self.hit_points
    }
}

fn main() {
    let billy = Character::new("Billy".to_string(), 9, 8, 7, 10, 19, 19, 5, Alignment::Good)
    let brandy = Character::new("Brandy".to_string(), 9, 8, 7, 10, 19, 19, 5, Alignment::Good)
    println!("Now Billy has {} hit points", billy.checked_sub(10).unwrap());

    let mut hit_points_vec = vec![];
    hit_points_vec.push(*billy);
    hit_points_vec.push(*brandy);

    println!("All our hit points are: {hit_points_vec:?}");
}