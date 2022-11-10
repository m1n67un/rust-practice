// rgb

fn match_colors(rgb: (u32, u32, u32)) {
    match rgb {
        (r, _, _) => r < 10 => println!("Not much red"),
        (_, g, _) => g < 10 => println!("Not much green"),
        (_, _, b) => b < 10 => println!("Not much blue"),
        _ =>  println!("Every color has at least 10")

    }
}

fn main() {
    let first = (200, 0, 0);
    let second = (50, 50, 50);
    let third = (200, 50, 0);

    match_colors(first);
    match_colors(second);
    match_colors(third);

    let num = 10;
    let some_variable = match num {
        10 => 8,
        _ => "Not ten" //: "Not ten" > String 형태가 아닌 integer 타입을 사용하여야 오류가 나지 않는다.
    };

    let num = 9;
    let some_variable = if num == 10 { 8 } else { "Something else"/* else return 값이 String이 아닌 Integer를 사용해야 오류가 발생하지 않는다. */ };
}