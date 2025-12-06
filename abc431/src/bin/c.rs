use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        mut h: [i32; n],
        mut b: [i32; m],
    };
    h.sort_by(|a, b| b.cmp(a));
    b.sort_by(|a, b| b.cmp(a));
    let mut h = VecDeque::from(h);

    let mut count = 0;
    for bi in b {
        while let Some(hj) = h.pop_front() {
            if bi >= hj {
                count += 1;
                break;
            }
        }
    }

    if count >= k {
        println!("Yes");
    } else {
        println!("No");
    }
}
