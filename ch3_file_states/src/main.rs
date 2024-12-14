use core::panic;
use std::usize;

#[derive(Debug, PartialEq)]
enum FileState {
    Open,
    Close,
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
    state: FileState,
}

impl File {
    fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Close,
        }
    }

    fn read(&self, save_to: &mut Vec<u8>) -> Result<usize, String> {
        if self.state != FileState::Open {
            return Err(String::from("file must be open before read"));
        }
        let mut tmp = self.data.clone();
        let read_length = tmp.len();
        save_to.reserve(read_length);
        save_to.append(&mut tmp);

        Ok(read_length)
    }
}

fn open(mut f: File) -> Result<File, String> {
    f.state = FileState::Open;
    Ok(f)
}

fn close(mut f: File) -> Result<File, String> {
    f.state = FileState::Close;
    Ok(f)
}

fn main() {
    let mut f5 = File::new("5.txt");

    let mut buffer: Vec<u8> = vec![];

    if f5.read(&mut buffer).is_err() {
        eprintln!("Error Checking is Working");
    }

    f5 = match open(f5) {
        Ok(res) => res,
        Err(e) => panic!("{}", e),
    };
    let f5_length = match f5.read(&mut buffer) {
        Ok(res) => res,
        Err(e) => panic!("{}", e),
    };

    f5 = match close(f5) {
        Ok(res) => res,
        Err(e) => panic!("{}", e),
    };

    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", f5);
    println!("{} is {} bytes longs", f5.name, f5_length);
    println!("{}", text)
}
