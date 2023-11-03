fn main() {
    proconio::input! {
        a: [i32; 2],
    }
    if (0..4).contains(&(a[0] - a[1])) || (0..3).contains(&(a[1] - a[0])) {
        println!("Yes");
    } else {
        println!("No");
    }
}
