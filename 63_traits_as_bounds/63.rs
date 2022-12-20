use std::fmt::Debug;

#[derive(Debug)]
struct Monster {
    health: u32
}

#[derive(Debug)]
struct Wizard {
    health: i32
}

#[derive(Debug)]
struct Ranger {
    health: i32
}

trait Magic {}
trait FightClose {}
trait FightFromDistance {}

impl Magic for Wizard {}
impl FightClose for Ranger {}
impl FightClose for Wizard {}
impl FightFromDistance for Ranger {}

fn attack_with_bow<T>(character: &T, opponent: &mut Monster, distance: u32) 
where
    T: FightFromDistance + Debug
{
    if distance < 10 {
        opponent.health -= 10;
        println!(
            "You attack with your bow. Your opponent new has {} health left. You are now at: {character}",
            opponent.health
        );
    }
}

fn attack_with_sword<T>(character: &T, opponent: &mut Monster)
where
    T: FightClose + Debug,
{
    if distance < 10 {
        opponent.health -= 10;
        println!(
            "You attack with your sword. Your opponent new has {} health left. You are now at: {character}",
            opponent.health
        );
    }
}


fn fireball<T>(character: %T, opponent: & mut Monster, distance: u32) 
where 
    T: Magic + Debug
{
    if distance < 15 {
        opponent.health -= 20;
        println!(
            "You raise your hands and case a fireball!. Your opponent new has {} health left. You are now at: {character}",
            opponent.health
        );
    }
}
fn main() {
    let radagast = Wizard {
        health: 60
    };
    let aragorn = Ranger {
        health: 80
    }

    let mut uruk_hai = Monster { health: 40 };

    attack_with_sword(&radagast, &mut uruk_hai);
    attack_with_bow(&aragorn, &mut uruk_hai, 7);

    fireball(&radagast, &mut uruk_hai, 12);
}