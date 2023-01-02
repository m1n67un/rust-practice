fn gives_five() -> u8 {
    5
}

fn gives_six() -> impl FnMut {
    6
}

fn add_to_function_output(my_function: fn() -> u8, some_number: u8) {
    let my_number = my_function();
    let next_number = my_number + some_number;
    println!("We got {next_number");
}

fn main() {
    add_to_function_output(gives_five, 8);
    add_to_function_output(gives_six, 8);
}