use proconio::input;

fn main() {
    input! {
        (n, k): (usize, i32),
        a: [i32; n],
        b: [i32; n],
    };
    let diff = a.iter().zip(b).fold(0, |acc, (&ai, bi)| { acc + (ai - bi).abs() });
    if diff > k {
        println!("No");
        return;
    }
    if (k - diff) % 2 == 1 {
        println!("No");
        return;
    }
    println!("Yes");
}
