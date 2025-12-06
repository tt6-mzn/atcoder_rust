use proconio::input;

fn main() {
    input! {
        h: i32,
        b: i32,
    };
    println!("{}", (h - b).max(0));
}
