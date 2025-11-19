use ac_library::ModInt1000000007 as Mint;
use proconio::input;

fn main() {
    input! {
        n: usize,
        l: usize,
    }

    // dp[i] := i段目から出発し、n段目に到達する方法の数
    let mut dp = vec![Mint::new(0); n + 1];
    dp[0] = Mint::new(1);
    for i in 0..n {
        let dpi = dp[i];
        dp[i + 1] += dpi;
        if i + l <= n {
            dp[i + l] += dpi;
        }
    }
    println!("{}", dp[n].val());
}
