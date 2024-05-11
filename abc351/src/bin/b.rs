use itertools::iproduct;
use proconio::marker::Chars;

fn main() {
    proconio::input! {
        n: usize,
        a: [Chars; n],
        b: [Chars; n],
    }
    for (i, j) in iproduct!(0..n, 0..n) {
        if a[i][j] != b[i][j] {
            println!("{} {}", i + 1, j + 1);
            return;
        }
    }
}
