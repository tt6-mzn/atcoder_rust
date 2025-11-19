use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
        c: [usize; n],
    };
    let mut ca = vec![0; 46];
    let mut cb = vec![0; 46];
    let mut cc = vec![0; 46];
    for i in 0..n {
        ca[a[i] % 46] += 1;
        cb[b[i] % 46] += 1;
        cc[c[i] % 46] += 1;
    }
    let mut ans: i64 = 0;
    for i in 0..46 {
        for j in 0..46 {
            for k in 0..46 {
                if (i + j + k) % 46 == 0 {
                    ans += ca[i] * cb[j] * cc[k];
                }
            }
        }
    }
    println!("{}", ans);
}
