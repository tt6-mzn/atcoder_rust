use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [[i32; n]; n],
        m: usize,
        xy: [(usize, usize); m],
    };
    let mut rumors = vec![vec![false; n]; n];
    let rumors = {
        for (i, j) in xy {
            let i = i - 1;
            let j = j - 1;
            rumors[i][j] = true;
            rumors[j][i] = true;
        }
        rumors
    };
    let mut ans = None;
    for perm in (0..n).permutations(n)  {
        if perm.windows(2).any(|w| {
            let i = w[0];
            let j = w[1];
            rumors[i][j]
        }) {
            continue;
        }
        let time = perm.iter().enumerate().fold(0, |acc, (j, &i)| {
            acc + a[i][j]
        });
        ans = Some(ans.map_or(time, |t| time.min(t)));
    }
    if let Some(time) = ans {
        println!("{}", time);
    } else {
        println!("-1");
    }
}
