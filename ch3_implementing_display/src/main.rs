use std::fmt::{self, Display};

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
enum FileState {
    Close,
    Open,
}

impl Display for FileState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FileState::Open => write!(f, "OPEN"),
            FileState::Close => write!(f, "CLOSE"),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
    state: FileState,
}

impl Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{} ({})>", self.name, self.state)
    }
}

impl File {
    fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            data: vec![],
            state: FileState::Close,
        }
    }
}

fn main() {
    let f6 = File::new("6.txt");
    println!("{:?}", f6);
    println!("{}", f6);
}
