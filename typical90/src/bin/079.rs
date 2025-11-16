use proconio::input;

fn main() {
    input! {
        (h, w): (usize, usize),
        mut a: [[i64; w]; h],
        b: [[i64; w]; h],
    };
    let mut ans = 0;
    for i in 0..(h - 1) {
        for j in 0..(w - 1) {
            let op = b[i][j] - a[i][j];
            a[i][j] += op;
            a[i][j + 1] += op;
            a[i + 1][j] += op;
            a[i + 1][j + 1] += op;
            ans += op.abs();
        }
    }
    for i in 0..h {
        for j in 0..w {
            if a[i][j] != b[i][j] {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
    println!("{}", ans);
}
