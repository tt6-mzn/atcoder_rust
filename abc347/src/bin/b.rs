use std::collections::HashSet;
use proconio::marker::Chars;

fn main() {
    proconio::input! {
        s: Chars
    }
    let mut st = HashSet::new();
    for i in 0..s.len() {
        for j in (i + 1)..=s.len() {
            st.insert(s[i..j].iter().copied().collect::<Vec<char>>());
        }
    }
    println!("{}", st.len());
}
