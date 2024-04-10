use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(i32, i32); n]
    }
    for (xi, yi) in xy.iter() {
        let mut ans = 0;
        let mut max_d = 0;
        for (j, (xj, yj)) in xy.iter().enumerate() {
            let d = (xi - xj).pow(2) + (yi - yj).pow(2);
            if d > max_d {
                max_d = d;
                ans = j;
            }
        }
        println!("{}", ans + 1);
    }
}
