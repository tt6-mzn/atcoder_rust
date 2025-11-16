use num::Integer;
use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    };
    let g = a.gcd(&b).gcd(&c);
    println!("{}", a / g - 1 + b / g - 1 + c / g - 1);
}
