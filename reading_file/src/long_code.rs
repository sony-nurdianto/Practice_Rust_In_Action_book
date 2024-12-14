use core::{panic, str};
use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

pub fn read_long_code() {
    let path = Path::new("readme.md");
    let file = match File::open(path) {
        Ok(v) => v,
        Err(e) => panic!("{}", e),
    };

    let mut buffer = BufReader::new(file);

    let mut reader: Vec<u8> = Vec::new();

    match buffer.read_to_end(&mut reader) {
        Ok(bit) => bit,
        Err(e) => panic!("{}", e),
    };

    match str::from_utf8(&reader) {
        Ok(v) => {
            let content = String::from(v);
            for line in content.lines() {
                println!("{} ({} bytes loing)", line, line.len());
            }
        }
        Err(e) => println!("Failed to decode file as UTF-8: {}", e),
    }
}
