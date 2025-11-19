use num_integer::gcd;
use proconio::input;

fn main() {
    input! {
        mut a: i64,
        mut b: i64,
    };
    let g = gcd(a, b);
    a /= g;
    if let Some(ans) = a.checked_mul(b) {
        if ans <= 10i64.pow(18) {
            println!("{}", ans);
            return;
        }
    }
    println!("Large");
}
