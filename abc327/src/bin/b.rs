fn main() {
    proconio::input! {
        b: u64,
    }
    let mut a: u64 = 1;
    while let Some(mut _t) = a.checked_pow(a as u32) {
        if a == b {
            println!("{a}");
            return;
        }
        a += 1;
    }
    println!("-1");
}
