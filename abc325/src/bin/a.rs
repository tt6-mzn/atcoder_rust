fn main() {
    proconio::input! {
        s: String,
        _t: String,
    }
    println!("{}", s.chars().chain(" san".chars()).collect::<String>());
}