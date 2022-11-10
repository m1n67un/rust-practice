enum Seasons {
    Spring,
    Summer,
    Autumn,
    Winter
}

fn main() {
    use Seasons::*;

    let four_seasons = vec![Spring, Summer, Autumn, Winter];
    for season in four_seasons {
        println!("The number is: {}", season as u32);
    }
}