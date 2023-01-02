use std::sync::{Mutex, RwLock};

fn main() {
    /*
    let my_mutex = Mutex::new(5);

    let mut mutex_changer = my_mutex.lock().unwrap();
    let mut other_mutex_changer = my_mutex.try_lock();

    if let Ok(value) = other_mutex_changer {
        println!("The mutexchanger has: {value}");
    } else {
        println!("Didn't get a lock");
    } 

    println!("{my_mutex:?}");
    */

    let my_rwlock = RwLock::new(5);

    let read1 = my_rwlock.read().unwrap();
    let read2 = my_rwlock.read().unwrap();

    println!("{read1:?}, {read2:?}");
}