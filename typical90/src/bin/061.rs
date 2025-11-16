use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        q: usize,
    };
    let mut deck = VecDeque::new();
    for _ in 0..q {
        input! {
            t: u8,
            x: i32,
        };
        match t {
            1 => deck.push_front(x),
            2 => deck.push_back(x),
            3 => println!("{}", deck[(x - 1) as usize]),
            _ => unreachable!(),
        };
    }
}
