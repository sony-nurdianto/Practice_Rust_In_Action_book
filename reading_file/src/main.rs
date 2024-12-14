use std::fs::File;
use std::io::{BufRead, BufReader};

mod long_code;

fn main() {
    let f = File::open("readme.md").unwrap();
    let reader = BufReader::new(f);

    for line_ in reader.lines() {
        let line = line_.unwrap();
        println!("{} ({} bytes long)", line, line.len());
    }

    long_code::read_long_code()
}
