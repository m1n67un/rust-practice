enum Mood {
    Happy,
    Sleepy,
    NotBad,
    Angry
}

fn match_mood(mood: &Mood) -> i32 {
    /*
    let happiness_level = match mood {
        Mood::Happy => 10,
        Mood::Sleepy => 6,
        Mood::NotBad => 7,
        Mood::Angry => 2
    };
    happiness_level
    */
    use Mood::*;

    match mood {
        Happy => 10,
        Sleepy => 6,
        NotBad => 7,
        Angry => 2
    }
}

fn main() {
    let my_mood = Mood::NotBad;
    let happiness_level = match_mood(&my_mood);
    println!("Out of 1 to 10, my happiness is {}", happiness_level);
}