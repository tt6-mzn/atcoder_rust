use std::collections::BTreeMap;

use proconio::marker::Chars;

fn main() {
    proconio::input! {
        s: Chars
    }
    let mut counter = BTreeMap::new();
    for c in s.iter() {
        counter.entry(*c)
            .and_modify(|v| *v += 1)
            .or_insert(1usize);
    }
    let mut ans = s.len() * (s.len() - 1) / 2;
    let mut id = 0usize;
    for (_, v) in counter {
        if v >= 2 {
            id = 1;
        }
        ans -= v * (v - 1) / 2;
    }
    println!("{}", ans + id);
}
