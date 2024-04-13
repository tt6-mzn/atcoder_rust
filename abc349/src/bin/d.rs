fn main() {
    proconio::input! {
        left: i64,
        mut right: i64
    }
    let mut m = 0i64;
    let mut ans: Vec<(i64, i64)> = Vec::new();
    while left < right {
        let mut i = 0;
        while (right >> i) & 1 == 0 {
            i += 1;
        }
        // j * 2^i = right
        let mut j = right >> i;
        // if j == 1 {
        //     j = 2;
        //     i -= 1;
        // }
        let mut l = (j - 1) << i;
        while l < left {
            i -= 1;
            j *= 2;
            l = (j - 1) << i;
        }
        ans.push((l, right));
        m += 1;
        right = l;
    }
    ans.reverse();

    println!("{}", m);
    for (l, r) in ans {
        println!("{} {}", l, r);
    }
}
