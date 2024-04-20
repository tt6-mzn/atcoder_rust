use std::collections::BTreeSet;

use itertools::iproduct;

fn main() {
    proconio::input! {
        n: usize,
        a: [i32; n],
        m: usize,
        b: [i32; m],
        l: usize,
        c: [i32; l]
    }
    let mut st = BTreeSet::new();
    for (ai, bi, ci) in iproduct!(a, b, c) {
        st.insert(ai + bi + ci);
    }

    proconio::input! {
        q: usize,
        x: [i32; q]
    }
    for xi in x {
        if st.contains(&xi) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
