use std::borrow::Cow;

#[derive(Debug)]
struct User<'a> {
    name: Cow<'a, str>
}

fn main() {
    let user_1 = User {
        name: name_1.into()
    };

    let user_2 = User {
        name: name_2.to_string().into()
    };

    user_1.is_borrowed();

    user_1.name.to_mut().push('!');
    user_1.is_borrowed();
}