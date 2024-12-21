#[allow(arithmetic_overflow)]
fn main() {
    let (a, b) = (225_u8, 200_u8);
    let c = a + b;
    print!("225 + 200 = {}", c);
}
