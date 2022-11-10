fn main() {
    let mut num = 9;
    let num_ref = &mut num; //: &mut 와 같은 reference가 2개 있으면 *도 2개, 1개가 있으므로 아래 num_ref 앞 * 1개

    *num_ref = 10;

    println!("Number is now {}", num);
}