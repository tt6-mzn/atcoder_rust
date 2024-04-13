use itertools::Itertools;

fn main() {
    proconio::input! {
        n: usize,
        a: i32,
        b: i32,
        d: [i32; n]
    }
    let mut d = d
        .iter()
        .map(|di| di % (a + b))
        .sorted()
        .dedup()
        .inspect(|di| eprintln!("{}", di))
        .collect::<Vec<i32>>();
    d.push(d[0] + a + b);
    for i in 0..d.len()-1 {
        if d[i + 1] - d[i] > b {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
