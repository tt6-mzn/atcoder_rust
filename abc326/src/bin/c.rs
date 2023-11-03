fn main() {
    proconio::input! {
        n: usize,
        m: i64,
        mut a: [i64; n],
    }
    a.sort();

    let mut l: usize = 0;
    let mut r = 0;
    let mut ans = 0;
    while l < n {
        while r < n && a[r] < a[l] + m {
            r += 1;
        }
        ans = std::cmp::max(ans, r - l);
        l += 1;
    }
    println!("{ans}");
}
