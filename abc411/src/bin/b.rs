use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        d: [i32; n - 1],
    };
    for i in 0..(n - 1) {
        let mut ans = Vec::new();
        for j in (i + 1)..n {
            ans.push(d[i..j].iter().sum::<i32>());
        }
        println!("{}", ans.iter().join(" "));
    }
}
