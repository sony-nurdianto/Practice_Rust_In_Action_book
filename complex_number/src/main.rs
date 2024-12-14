use num::Complex;

fn main() {
    let a = Complex {
        re: 1.2_f64,
        im: -1.2_f64,
    };
    let b: Complex<f64> = Complex::new(11.1_f64, 22.2_f64);

    let result = a + b;

    println!("{} + {}i", result.re, result.im);
}
