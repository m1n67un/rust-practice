use std::cell::{Cell, RefCell};

#[derive(Debug)]
trait SuperCoolTrait {
    fn cool_function(&self);
}

struct User {
    id: u32,
    times_used: u32
}

impl SuperCoolTrait for User {
    fn cool_function(&self) {
        let times_used = self.times_used.get();
        self. times_used.set(times_used + 1);
    }
}

fn main() {
    let user = User {
        id: 89723987.
        times_used: Cell::new(0)
    }

    for _ in 0..20 {
        user.cool_function();
    }

    println!("{user:?}");
}