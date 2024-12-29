use std::collections::HashMap;

fn main() {
    let mut capital = HashMap::new();

    capital.insert("Cook Islands", "Avarua");
    capital.insert("Fiji", "Suva");
    capital.insert("Kiribati", "South Tarawa");
    capital.insert("Tonga", "Nuku'alofa");
    capital.insert("Tuvalu", "Funafuti");

    let tongan_capital = capital["Tonga"];

    println!("Capital of Tonga is : {}", tongan_capital);
}
