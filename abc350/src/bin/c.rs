fn main() {
    proconio::input! {
        n: usize,
        mut a: [usize; n]
    }
    let mut vtoi = vec![0usize; n + 1];
    for (i, v) in a.iter().enumerate() {
        vtoi[*v] = i + 1;
    }
    let mut ans = Vec::new();
    for i in 1..n + 1 {
        let j = vtoi[i];
        if i == j {
            continue;
        }
        ans.push((i, j));
        vtoi[i] = i;
        vtoi[a[i - 1]] = j;
        (a[i - 1], a[j - 1]) = (a[j - 1], a[i - 1]);
    }
    println!("{}", ans.len());
    for (i, j) in ans {
        println!("{i} {j}");
    }
}
