#[allow(unused_variables)]
#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

impl File {
    fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            data: vec![],
        }
    }

    fn new_with_data(name: &str, data: &Vec<u8>) -> Self {
        Self {
            name: String::from(name),
            data: data.clone(),
        }
    }
}

fn open(f: &mut File) -> bool {
    true
}

fn close(f: &mut File) -> bool {
    true
}

fn read(f: &File, save_to: &mut Vec<u8>) -> usize {
    let mut tmp = f.data.clone();
    let read_length = tmp.len();

    save_to.reserve(read_length);
    save_to.append(&mut tmp);

    read_length
}

fn main() {
    let f3_data = vec![114, 117, 115, 116, 33];
    let mut f3 = File::new_with_data("f2.txt", &f3_data);

    let mut buffer: Vec<u8> = Vec::new();

    open(&mut f3);
    let f3_length = read(&f3, &mut buffer);
    close(&mut f3);

    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", f3);
    println!("{} is {} bytes long", &f3.name, &f3_length);
    println!("{}", text);
}
