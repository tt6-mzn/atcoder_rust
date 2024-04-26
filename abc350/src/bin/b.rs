fn main() {
    proconio::input! {
        n: usize,
        q: usize,
        t: [usize; q]
    }
    let mut tooth = vec![true; n];
    for ti in t {
        let ti = ti - 1;
        tooth[ti] = !tooth[ti];
    }
    println!("{}", tooth.iter().filter(|e| **e).count());
}
