mod client {
    pub struct InternetClinet {
        pub client_id: u32,
    }
}

use client::InternetClinet;

#[derive(Debug)]
struct Customer<'a> {
    money: u32,
    name: &'a str,
    client: &'a InternetClinet
}

use str::fmt;

impl fmt::Debug for Customer<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Customer")
            .field("money", &self.money)
            .field("name", &self.name)
            .field("Client", &"Client")
            .finish()
    }
}

fn main() {
    let client = client::InternetClinet {
        client_id: 0
    };

    let customer1 = Customer {
        money: 6876,
        name: "Billy",
        client: &client
    };

    println!("customer1:?");
}