fn main() {
    proconio::input! {
        _n: usize,
        s: String,
    }
    if s.contains("ab") || s.contains("ba") {
        println!("Yes");
    } else {
        println!("No");
    }
}
