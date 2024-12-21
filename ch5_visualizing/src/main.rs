const BIAS: i32 = 127;
const RADIX: f32 = 2.0;

fn to_parts(n: f32) -> (u32, u32, u32) {
    let bits = n.to_bits();
    let sign = (bits >> 31) & 1;
    let exponent = (bits >> 23) & 0xfff;
    let frac = bits & 0x7fffff;

    (sign, exponent, frac)
}

fn decode(sign: u32, exponent: u32, fraction: u32) -> (f32, f32, f32) {
    let signed_1 = (-1.0_f32).powf(sign as f32);
    let exponent = (exponent as i32) - BIAS;
    let exponent = RADIX.powf(exponent as f32);
    let mut mantisa: f32 = 1.0;

    for i in 0..23 {
        let mask = 1 << i;
        let one_at_bit_i = fraction & mask;
        if one_at_bit_i != 0 {
            let i_ = i as f32;
            let weight = 2_f32.powf(i_ - 23.0);
            mantisa += weight
        }
    }

    (signed_1, exponent, mantisa)
}

fn from_part(sign: f32, exponent: f32, mantisa: f32) -> f32 {
    sign * exponent * mantisa
}

fn main() {
    let n: f32 = 42.42;
    let (sign, exponent, frac) = to_parts(n);
    let (sign_, exponent_, mant) = decode(sign, exponent, frac);
    let n_ = from_part(sign_, exponent_, mant);

    println!("{} -> {}", n, n_);
    println!("{} -> {}", n, n_);
    println!("field    |  as bits | as real number");
    println!("sign     |        {:01b} | {}", sign, sign_);
    println!("exponent | {:08b} | {}", exponent, exponent_);
    println!("mantissa | {:023b} | {}", frac, mant);
}
