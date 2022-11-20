use num::complex::Complex;

fn main() {
    let a = Complex { re: 2.1, im: -1.2 };
    let b = Complex { re: 11.1, im: 22.2 };
    let result = a + b;
    print!("{} + {}i", result.re, result.im);
}
