use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }

    if a.iter().all(|e| *e == a[0]) {
        println!("Yes");
    } else {
        println!("No");
    }
}