fn main() {
    proconio::input! {
        _: i32,
        x: i32,
        y: i32,
        z: i32
    }
    if x.min(y) <= z && z <= x.max(y) {
        println!("Yes");
    } else {
        println!("No");
    }
}
