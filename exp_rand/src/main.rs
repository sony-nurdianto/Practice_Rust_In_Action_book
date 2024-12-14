use rand::prelude::*;

fn main() {
    let denominator = 100;
    let random_fraction = thread_rng().gen_ratio(1, denominator);
    println!("Fraksi acak: {}", random_fraction);
}
