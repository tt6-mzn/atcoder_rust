use itertools::Itertools;
use proconio::marker::Chars;

fn main() {
    proconio::input! {
        s: Chars
    }
    let mut ans = Vec::new();
    let mut flg = true;
    for c in s {
        if c == '|' {
            flg = !flg;
            continue;
        }
        if flg {
            ans.push(c);
        }
    }
    if ans.len() > 0 {
        println!("{}", ans.iter().join(""));
    }
}