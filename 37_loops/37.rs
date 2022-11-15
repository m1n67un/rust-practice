// loops

fn main() {

    let mut counter = 0;
    let mut counter2 = 0;

    /*
    loop {
        loop {
            counter += 1;
            println!("The counter is: {}", counter);
            if counter == 5 {
                break; // Loop 빠져나오기
            }
        }
    }
    */
    'first_loop: loop {
        counter += 1;
        println!("The counter is now: {}", counter);
        if counter > 9 {
            println!("Now entering the second loop");
            
            'second_loop: loop {
                println!("The second counter is now: {}", counter2);
                counter2 += 1;
                if counter2 == 3 {
                    break 'first_loop;
                }
            }
        }
    }
}