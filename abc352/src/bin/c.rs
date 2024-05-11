fn main() {
    proconio::input! {
        n: usize,
        ab: [(i64, i64); n],
    }
    let heads = ab.iter().map(|p| p.1 - p.0).collect::<Vec<i64>>();
    println!("{}", ab.iter().fold(0, |acc, p| acc + p.0) + heads.iter().max().unwrap());
}
