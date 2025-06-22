// dp[i][h][m] = i番目までのモンスターを倒す時に、体力h, 魔力m を残せるか
// dp[0][H][M] = true
// dp[i][h][m] = dp[i-1][h+ai][m] or dp[i-1][h][m+bi]

// dp[i][h] = i番目までのモンスターを、体力 h を残して倒すときに残せる魔力の最大値
// dp[0][H] = M
// dp[i][h] = max(dp[i-1][h+ai], dp[i-1][h]-bi)

use num::ToPrimitive;
use proconio::input;
use std::cmp::max;

fn main() {
    input! {
        n: usize,
        h: usize,
        m: i32,
        ab: [(usize, i32); n],
    };
    let mut dp = vec![vec![-1; h + 1]; n + 1];
    dp[0][h] = m;
    for i in 1..n+1 {
        let (ai, bi) = ab[i - 1];
        for hh in 0..h+1 {
            if hh + ai <= h.to_usize().unwrap() {
                dp[i][hh] = max(dp[i - 1][hh + ai], dp[i - 1][hh] - bi)

            } else {
                dp[i][hh] = dp[i - 1][hh] - bi;
            }
        }
    }
    let mut ans = 0;
    for i in 0..n+1 {
        for hh in 0..h+1 {
            if dp[i][hh] >= 0 {
                ans = i;
            }
        }
    }
    println!("{}", ans);
}
