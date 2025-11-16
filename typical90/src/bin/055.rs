use ac_library::ModInt as Mint;
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (n, p, q): (usize, u32, u32),
        a: [i32; n],
    }
    Mint::set_modulus(p);
    let a = a.iter().map(|&ai| { Mint::new(ai) }).collect::<Vec<Mint>>();

    let mut ans = 0;
    for comb in a.iter().combinations(5) {
        let prod = comb
            .iter()
            .fold(Mint::new(1), |acc, &&x| acc * x);
        if prod.val() == q {
            ans += 1;
        }
    }
    println!("{}", ans);
}
