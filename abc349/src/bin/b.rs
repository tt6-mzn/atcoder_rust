use proconio::marker::Chars;
use std::collections::BTreeMap;

fn main() {
    proconio::input! {
        s: Chars
    }
    let mut count = BTreeMap::new();
    for c in s {
        count.entry(c).and_modify(|e| *e += 1).or_insert(1);
    }
    let mut map = BTreeMap::new();
    for (_, n) in count {
        map.entry(n).and_modify(|e| *e += 1).or_insert(1);
    }
    for (_, n) in map {
        if n != 2 {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
