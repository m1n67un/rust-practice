/*
fn print_number(one: i32, two: i32) {
    let multipleied = one * two;
    println!("{}", multipleied);
    one * two
}

fn main() {
    print_number(9, 8);
}
*/

fn give_number(one: i16, two: i16) -> i16 {
    let multipleied_by_ten = {
        let first_number = 10;
        first_number * one * two
    };
    multipleied_by_ten
}

fn main() {
    let num = give_number(9, 8);
    println!("{}", num);
}