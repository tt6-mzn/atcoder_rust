fn main() {
    proconio::input! {
        n: usize,
        a: [i32; n - 1]
    }
    let ans = -a.iter().sum::<i32>();
    println!("{ans}");
}
