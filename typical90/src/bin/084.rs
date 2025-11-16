use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    };
    let mut next = vec![None; n];
    for i in (1..n).rev() {
        if s[i - 1] != s[i] {
            next[i - 1] = Some(i);
        } else {
            next[i - 1] = next[i];
        }
    }

    let mut ans = 0;
    for l in 0..(n - 1) {
        if let Some(r_min) = next[l] {
            ans += n - r_min;
        }
    }
    println!("{ans}");
}
