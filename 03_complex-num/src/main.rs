use num::complex::Complex;
fn main() {
    let a = Complex { re: 2.1, im: -1.2 };
    let b = Complex::new(11.1, 22.2);
    let result = a + b;
    println!("{} + {}i", result.re, result.im);

    let x = '😀';
    println!("{}", x);
    println!("{}", std::mem::size_of_val(&x));
    let y = ();
    println!("{}", std::mem::size_of_val(&y));
    let f = true;
    if f {
        println!("aaa");
    }
}
