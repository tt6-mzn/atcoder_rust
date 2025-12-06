use proconio::input;

fn main() {
    input! {
        n: i64,
        (a, b, c): (i64, i64, i64),
    };
    let mut ans = std::i64::MAX;
    for na in 0..10000 {
        for nb in 0..(10000 - na) {
            let r = n - a * na - b * nb;
            if r < 0 {
                continue;
            }
            if r % c != 0 {
                continue;
            }
            let nc = r / c;
            ans = ans.min(na + nb + nc);
        }
    }
    println!("{}", ans);
}
