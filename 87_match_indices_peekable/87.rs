/*
fn user_input() -> String {
    "me@me.com".to_string()
}

fn main() {
    let rules = "Rule number 1: No fighting.
Rule number 2: Go to bed at 8 pm.
Rule number 3: Wake up at 6 am.";

    let rule_locations = rules.match_indices("Rule").collect::<Vec<(_, _)>>();
    println!("Rule locations: {rule_locations:?}");
}
*/

fn main() {
    let just_numbers = vec![1, 5, 100];

    let mut number_iter = just_numbers.iter().peekable();

    for _ in 0..3 {
        println!("I love the number {}", number_iter.peek().unwrap());
        println!("I really love the number {}", number_iter.peek().unwrap());
        number_iter.next();
    }
}