use proconio;

fn main() {
    proconio::input! {
        n: usize,
        a: [i32; n],
        k: i32,
    }
    let mut ans = 0;
    for ai in a {
        if k <= ai {
            ans += 1;
        }
    }
    println!("{ans}");
}
