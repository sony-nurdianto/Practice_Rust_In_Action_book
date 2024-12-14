use std::usize;

#[derive(Debug)]
struct File;

trait Read {
    fn read(self: &Self, save_to: &mut Vec<u8>) -> Result<usize, String>;
}

#[allow(unused_variables)]
impl Read for File {
    fn read(self: &File, save_to: &mut Vec<u8>) -> Result<usize, String> {
        Ok(0)
    }
}

fn main() {
    let f = File {};

    let mut buffer: Vec<u8> = vec![];

    let n_bytes = f.read(&mut buffer);

    println!("{} is {:?} bytes longs", n_bytes.unwrap(), f);
}
