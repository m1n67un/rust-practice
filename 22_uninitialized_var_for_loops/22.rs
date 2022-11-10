fn loop_then_return(mut counter: i32) -> i32{
    loop {
        counter += 1;
        if counter % 50 == 0 {
            break;
        }
        counter
    }
}

fn main() {
    let num;
    {
        num = 0;
    }

    /*
    let num = {
        let x = 9;
        x + 9
    }
    */

    {
        let x = loop_then_return(43);
        num = x
    }
    println!("{}", num);

}