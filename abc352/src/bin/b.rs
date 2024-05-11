use itertools::Itertools;
use proconio::marker::Chars;

fn main() {
    proconio::input! {
        s: Chars,
        t: Chars,
    }
    let mut j = 0;
    let mut ans = Vec::new();
    for si in s {
        while si != t[j] {
            j += 1;
        }
        ans.push(j + 1);
        j += 1;
    }
    println!("{}", ans.iter().join(" "))
}
