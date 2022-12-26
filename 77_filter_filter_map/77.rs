/*
fn main() {
    let months = vec![
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December"
    ];

    let filetered_months = months
        .into_iter()
        .filter(|month| month.len()) < 5)
        .filter(|monnnth| monnnth.contains("u"))
        .collect::<Vec<&str>>();

        println!("{filetered_months}");
}
*/

struct Company {
    name: String,
    ceo: Option<String>
}

impl Company {
    fn new(name: &str, ceo: &str) -> Self {
        let ceo = match ceo {
            "" => None,
            ceo => Some(ceo.to_string())
        };

        Self {
            name: name.to_string,
            ceo
        }
    }

    fn get_ceo(&self) -> Option<String> {
        self.ceo.clone()
    }
}

fn main() {
    let company_vec = vec![
        Company::new("Umbrella Corporation", "Unknown"),
        Company::new("Ovintiv", "Doug Suttles"),
        Company::new("The Red-Headed League", ""),
        Company::new("Stark Enterprises", "")
    ];

    let all_the_ceos = company_vec
        .into_iter()
        .filter_map(|company| company.get_ceo())
        .collect::<vec<_>>();
    
    println!("{all_the_ceos:?}");
}