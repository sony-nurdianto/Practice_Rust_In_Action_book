use serde_json::json;

fn main() {
    let capitals = json!({
        "Cook Islands": "Avarua",
        "Fiji": "Suva",
        "Kiribati": "South Tarawa",
        "Niue": "Alofi",
        "Tonga": "Nuku Alofa",
        "Tuvalu": "Funafuti"
    });

    println!(
        "Capital of Tonga is : {}",
        capitals.get("Tonga").expect("Not Found")
    );
}
