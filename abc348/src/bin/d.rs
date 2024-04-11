use std::collections::VecDeque;
use std::cmp::max;

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        (h, w): (usize, usize),
        a: [Chars; h],
        n: usize,
    }
    let (s, t) = {
        let mut s = (0, 0);
        let mut t = (0, 0);
        for i in 0..h {
            for j in 0..w {
                if a[i][j] == 'S' {
                    s = (i, j);
                }
                if a[i][j] == 'T' {
                    t = (i, j);
                }
            }
        }
        (s, t)
    };
    let medicine = {
        let mut ret = vec![vec![0; w]; h];
        input! {
            rce: [(usize, usize, i32); n]
        }
        for (r, c, e) in rce {
            ret[r - 1][c - 1] = e;
        }
        ret
    };

    let mut dp = vec![vec![0; w]; h];
    let mut flg = vec![vec![false; w]; h];
    let mut que = VecDeque::new();
    dp[s.0][s.1] = medicine[s.0][s.1];
    flg[s.0][s.1] = true;
    que.push_back((s.0, s.1));
    while let Some((i, j)) = que.pop_front() {
        if dp[i][j] == 0 {
            continue;
        }
        for (di, dj) in [(0, 1), (-1, 0), (0, -1), (1, 0)] {
            let ni = i as i32 + di;
            let nj = j as i32 + dj;
            if ni < 0 || ni >= h as i32 {
                continue;
            }
            if nj < 0 || nj >= w as i32 {
                continue;
            }
            let ni = ni as usize;
            let nj = nj as usize;
            if a[ni][nj] == '#' {
                continue;
            }
            let ne = max(dp[i][j] - 1, medicine[ni][nj]);
            flg[ni][nj] = true;
            if dp[ni][nj] < ne {
                dp[ni][nj] = ne;
                que.push_back((ni, nj));
            }
        }
    }

    // for row in &flg {
    //     for c in row {
    //         print!("{}", if *c { "1" } else { "0" });
    //     }
    //     println!("");
    // }
    
    if flg[t.0][t.1] {
        println!("Yes");
    } else {
        println!("No");
    }
}