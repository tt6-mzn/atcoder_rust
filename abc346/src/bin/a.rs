use itertools::Itertools;

fn main() {
    proconio::input! {
        n: usize,
        a: [i32; n]
    }
    let mut b = Vec::new();
    for i in 0..n-1 {
        b.push(a[i] * a[i + 1]);
    }
    println!("{}", b.iter().map(|bi| bi.to_string()).join(" "));
}
