use core::panic;

use clap::{App, Arg};
use regex::Regex;

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
        .get_matches();

    let pattern = match args.value_of("pattern") {
        Some(p) => p,
        None => panic!("Pattern Is Not Found"),
    };

    let re = match Regex::new(pattern) {
        Ok(r) => r,
        Err(e) => panic!("{}", e),
    };

    let quote = "Every face, every shop, bedroom window, public-house, and
 dark square is a picture feverishly turned--in search of what?
 It is the same with books. What do we seek through millions of pages?";

    for line in quote.lines() {
        match re.find(line) {
            Some(_) => println!("{}", line.trim()),
            None => continue,
        }
    }
}
