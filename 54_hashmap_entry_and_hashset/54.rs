use std::collections::HashMap;

fn main() {
    let data = vec![
        ("male", 9),
        ("female", 5),
        ("male", 0),
        ("female", 6),
        ("female", 5),
        ("male", 10),
    ];

    let mut survey_hash = HashMap::new();

    for (item) in data { // (&str, i32) => tuple
        survey_hash.entry(item.0).or_insert(vec::new()).push(item.1);
    }

    /* 위와 ( = )
        for (gender, number) in data { // (&str, i32) => tuple
            survey_hash.entry(gender).or_insert(vec::new()).push(number);
        }
    */

    for (male_or_female, numbers) in survey_hash {
        println!("{:?}, {:?}", male_or_female, numbers);
    }
}


//HashSet => 있는지 없는지.
//BtreeSet => 순서도 중요하다면.

fn main() {
    let many_numbers = vec![
        94, 42, 59, 64, 32, 22, 38, 5, 59, 49, 15, 89, 74, 29, 14, 25, 26, 33, 43, 72, 78, 77, 62, 11, 5, 5, 6, 7, 8
    ];

    let mut nubmer_hashset = HashSet::new();

    for number in many_numbers {
        nubmer_hashset.insert(number);
    }

    let hashset_length = nubmer_hashset.len();
    println!("There are {} unique numbers, so we are missing {}.", hashset_length, 100 - hashset_length);

    let mut missing_vec = vec![];
    for number in 0..100 {
        if number_hashset.get(&number).is_none() {
            missing_vec.push(number);
        }
    }

    print!("It does not contain: ");
    for nubmer in missing_vec {
        print!("{}", number);
    }
}