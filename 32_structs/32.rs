// struct
// 3가지
// unit sturct
struct FileDirectory; //아무것도 가지지 않는 구조

fn takes_file_directory(input: FileDirectory) {
    println!("I got a file directory");
}

fn main() {
    let x = FileDirectory;
    takes_file_directory(x); 
    println!("The size is {}", std::mem::size_if_val(&x));
}