use itertools::Itertools;

fn main() {
    proconio::input! {
        n: usize,
        k: i64,
        a: [i64; n],
    }
    let mut ans = ((k + 1) * k) / 2;
    ans -= a.iter()
        .filter(|e| **e <= k)
        .sorted()
        .dedup()
        .sum::<i64>();
    println!("{ans}");
}
