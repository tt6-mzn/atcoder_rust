use itertools::Itertools;

fn main() {
    proconio::input! {
        n: usize,
        a: [[i32; n]; n],
    }
    for i in 0..n {
        let mut ans = Vec::new();
        for j in 0..n {
            if a[i][j] == 1 {
                ans.push((j + 1).to_string());
            }
        }
        println!("{}", ans.iter().join(" "));
    }
}
