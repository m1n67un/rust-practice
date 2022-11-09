fn give_age() -> i32 {
    /* return */ 30 //리턴을 쓸 필요가 없다. 마지막 줄 자동리턴
}

fn main() {
    let my_name = "David";
    let my_age = 30;
    println!("My name is {} and my age is {}", my_name, my_age);
    println!("My name is {} and my age is {}", my_name, give_age());

    let city = "Seoul";
    let year = 2022;
    let population = 9_123_123;
    println!("The city of {} in {} had a population of {}", city, year, population);

    println!("The city of {city} in {year} had a population of {population}",
        city = city,
        year = year,
        population = population
    );

    println!("The city of {0} in {1} had a population of {2}. I love {0}!",
        city,
        year,
        population
    );
    
    // string interpolation
    println!("The city of {city} in {year} had a population of {population}");
}