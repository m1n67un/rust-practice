enum ThingsInTheSky {
    Sun,
    Stars
}

fn create_skystate(time: i32) -> ThingsInTheSky {
    match time {
        6..=18 => ThingsInTheSky::Sun,
        _ => ThingsInTheSky::Stars
    }
}

fn check_skystate(state: &ThingsInTheSky) {
    match state {
        ThingsInTheSky::Sun => println!("I can see the sun"),
        ThingsInTheSky::Stars => println!("I can see te stars")
    }
}

fn main() {
    let time = 20;
    let sky_state = create_skystate(time);
    check_skystate(&sky_state);
}