use core::panic;

use rand::{thread_rng, Rng};

fn one_in(denominator: u32) -> bool {
    thread_rng().gen_ratio(1, denominator)
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

impl File {
    fn new(name: &str, data: &Vec<u8>) -> Self {
        Self {
            name: String::from(name),
            data: data.clone(),
        }
    }
}

fn read(f: &File, save_to: &mut Vec<u8>) -> Result<usize, String> {
    let mut tmp = f.data.clone();
    let read_length = tmp.len();
    save_to.reserve(read_length);
    save_to.append(&mut tmp);
    Ok(read_length)
}

fn open(f: File) -> Result<File, String> {
    if one_in(10_000) {
        let err_msg = String::from("Permisssion Denied");
        return Err(err_msg);
    }

    Ok(f)
}

fn close(f: File) -> Result<File, String> {
    if one_in(100_000) {
        let err_msg = String::from("Interupted by signal !!!");
        return Err(err_msg);
    }
    Ok(f)
}

fn main() {
    let f4_data: Vec<u8> = vec![114, 117, 115, 116, 33];
    let mut f4 = File::new("4.txt", &f4_data);

    let mut buffer: Vec<u8> = vec![];

    f4 = match open(f4) {
        Ok(f) => f,
        Err(e) => panic!("{}", e),
    };
    let f4_length = match read(&f4, &mut buffer) {
        Ok(res) => res,
        Err(e) => panic!("{}", e),
    };

    f4 = match close(f4) {
        Ok(res) => res,
        Err(e) => panic!("{}", e),
    };

    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", f4);
    println!("{} is {} bytes long", f4.name, f4_length);
    println!("{}", text)
}
