use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        p: Chars,
        l: usize,
    };
    if p.len() >= l {
        println!("Yes");
    } else {
        println!("No");
    }
}
