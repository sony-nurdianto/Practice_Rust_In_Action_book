#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

fn main() {
    let f1 = File {
        name: String::from("f1.txt"),
        data: vec![],
    };

    let f1_name = &f1.name;
    let f1_data = &f1.data.len();

    println!("{}", f1_name);
    println!("{}", f1_data);
}
