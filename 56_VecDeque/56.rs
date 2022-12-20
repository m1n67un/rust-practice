fn main() {
    let my_vec = vec![0; 600_000]; // .pop .push .remove(0)
    for i in 0..600000 {
        my_vec.remove(0);
    }
}

//: Vec 이용시 시간이 지연된다... 그럴 떄 VecDeque을 이용한다. 하지만 시간소요가 많이 걸리지 않는 작업에도 VecDeque을 사용하면 더 느릴 수도 있다.
use std:collections::VecDeque;

fn main() {
    let my_Vecdeq = VecDeque::form(vec![0; 600_000]);
    for i in 0..600000 {
        my_Vecdeq.pop_front();
    }
}