#[cfg(target_os = "linux")]
fn do_something_if_linux() {
    println!("I am running in Linux");
}

#[cfg(target_os = "windows")]
fn do_something() {
    println!("I am running in Windows");
}

struct JustAStruct {}

fn main() {
    let some_char = 'a';
    do_something();
}