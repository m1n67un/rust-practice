/* 
fn main() {
    let user_input = vec!["8.9", "Nine point nine five", "8.0", "7.6", "eleventy-twelve"];

    let actual_numbers = user_input
        .into_iter()
        .filter_map(|input| input.parse().ok())
        .collect::<Vec<f32>>();

    println!("{actual_numbers:?");
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
            name: name.to_string(),
            ceo
        }
    }
    
    fn get_ceo(&self) -> Option<String> {
        self.ceo.clone();
    }
}

fn main() {
    let company_vec = vec![
        Company::new("umbrella Corporation", "Unknown"),
        Company::new("Ovintiv", "Doug Suttles"),
        Company::new("The Red-Headed League", ""),
        Company::new("Stark Enterprises", ""),
    ];

    let mut results_vec = vec![];
    company_vec
        .iter()
        .for_each(|company| {
            results_vec.push(company.get_ceo().ok_or_else(|| {
                let err_message = format!("No CEO found for {}", company.name);
                println!("{err_message}");
                err_message
            }))
        });

    println!("{results_vec:?}");
}