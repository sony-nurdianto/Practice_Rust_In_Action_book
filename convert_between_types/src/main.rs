fn main() {
    let a: i32 = 10;
    let b: u16 = 100;

    let b: Result<i32, _> = b.try_into();

    match b {
        Ok(r) => {
            if a < r {
                println!("Ten is less than Hundred");
            }
        }
        _ => {
            println!("Hundred Is More than Ten");
        }
    }
}
