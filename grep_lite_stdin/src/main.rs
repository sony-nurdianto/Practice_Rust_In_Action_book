use core::panic;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

use clap::{App, Arg};
use regex::Regex;

fn proccess_lines<T>(reader: T, re: Regex)
where
    T: BufRead + Sized,
{
    for line_ in reader.lines() {
        let content: String = match line_ {
            Ok(v) => v,
            Err(e) => {
                eprintln!("{}", e);
                continue;
            }
        };
        match re.find(&content) {
            Some(_) => println!("{}", content),
            None => continue,
        }
    }
}

fn main() {
    let args = App::new("grep lite")
        .version("0.1")
        .about("searching for pattern")
        .arg(
            Arg::with_name("pattern")
                .help("The Pattern to Search For")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("input")
                .help("File To Search")
                .takes_value(true)
                .required(false),
        )
        .get_matches();

    let pattern = match args.value_of("pattern") {
        Some(p) => p,
        None => panic!("Pattern Is Not Found"),
    };

    let re = match Regex::new(pattern) {
        Ok(r) => r,
        Err(e) => panic!("{}", e),
    };

    let input = match args.value_of("input") {
        Some(i) => i,
        None => "-",
    };

    if input == "-" {
        let stdin = io::stdin();
        let reader = stdin.lock();
        proccess_lines(reader, re);
    } else {
        let path = Path::new(input);
        let file = match File::open(path) {
            Ok(f) => f,
            Err(e) => panic!("File Not Found : {}", e),
        };
        let reader = BufReader::new(file);

        proccess_lines(reader, re);
    }
}
