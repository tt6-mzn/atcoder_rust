use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i64; n],
    };

    let mut diff: Vec<i64> = a.windows(2).map(|w| { w[0] - w[1] }).collect();
    let mut cost: i64 = diff.iter().map(|e| { e.abs() }).sum();

    for _ in 0..q {
        input! {
            l: Usize1,
            r: Usize1,
            v: i64,
        };
        cost -= diff[l - 1].abs();
        diff[l - 1] -= v;
        cost += diff[l - 1].abs();
        if r < n - 1 {
            cost -= diff[r].abs();
            diff[r] += v;
            cost += diff[r].abs();
        }
        println!("{}", cost);
    }
}
