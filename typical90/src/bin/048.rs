use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        ab: [(i64, i64); n],
    }
    let mut scores = Vec::new();
    for (a, b) in ab {
        scores.push(b);
        scores.push(a - b);
    }
    scores.sort_by(|a, b| b.cmp(a));
    let ans: i64 = scores[0..k].iter().sum();
    println!("{}", ans);
}
