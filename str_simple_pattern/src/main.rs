fn main() {
    let search = "picture";
    let quote = "\
        Every face, every shop, bedroom window, public-house, and
        dark square is a picture feverishly turned--in search of what?
        It is the same with books.
        What do we seek through millions of pages?";
    let mut line_num = 1;
    let mut words: String = String::new();

    for (i, line) in quote.lines().enumerate() {
        if line.contains(search) {
            words.push_str(line.trim());
        }
        line_num = i;
    }
    println!("words: {}\nfound in line: {}", words, line_num);
}
