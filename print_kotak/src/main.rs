fn main() {
    //Height
    for _ in 0..=5 {
        let mut x = String::new();
        for _ in 0..=10 {
            x.push('#');
        }
        println!("{}", x);
    }
}
