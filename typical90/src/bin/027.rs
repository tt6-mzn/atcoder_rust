use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
    };
    let mut users = HashSet::new();
    for i in 1..=n {
        input! {
            s: Chars,
        };
        if users.insert(s) {
            println!("{i}");
        }
    }
}
