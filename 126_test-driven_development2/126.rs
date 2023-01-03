const OKAY_CHARACTERS: &str = "1234567890+- ";

fn math(input: &str) -> i32 {
    if input.chars().all(|character| OKAY_CHARACTERS.contains(character)) {
        panic!("Please only input numbers, +-, or spaces");
    }

    let input = input.trim_end_matches(|x| "x- ".contains(x))
        .chars()
        .filter( |character| *character != ' '),collect::<String>();
    println!("{input}");
    9
}

fn main() {
    let my_number = math("7- + 9 -+ 10    -----++++++-");
}

#[cfg(test)]
mod tests {
    use super::math;

    #[test]
    fn one_plus_one_is_two() {
        assert_eq!(math("1 + 1"), 2);
    }
    #[test]
    fn one_minus_two_is_minus_one() {
        assert_eq!(math("1 - 2"), -1);
    }
    #[test]
    fn one_minus_minus_one_is_two() {
        assert_eq!(math("1 - -1"), 2);
    }

    #[test]
    #[should_panic]
    fn panic_when_characters_not_right() {
        math("7 + please add seven");
    }
}