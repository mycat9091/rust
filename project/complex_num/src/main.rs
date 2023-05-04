use num::Complex;

fn main() {
    let a = Complex { re: 2.1, im: -1.2 };
    let b = Complex::new(11.1, 22.2);
    let result = a + b;
    let mut p = 13.14;
    p = 13.14_f32.round();
    println!("{}+{}i", result.re, result.im);
    println!("{}", p);
}
