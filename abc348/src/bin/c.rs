use proconio::input;
use std::cmp::{max, min};
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        ac: [(i32, i32); n]
    }
    let mut ca: HashMap<i32, i32> = HashMap::new();
    for (a, c) in ac {
        match ca.get_mut(&c) {
            Some(v) => {
                *v = min(*v, a);
            }
            None => {
                ca.insert(c, a);
            }
        }
    }
    let ans = ca
        .iter()
        .fold(0, |acc, a| max(acc, *a.1));
    println!("{ans}");
}
