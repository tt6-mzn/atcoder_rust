use ac_library::ModInt1000000007 as Mint;
use proconio::input;

fn main() {
    input! {
        n: usize,
        dices: [[u32; 6]; n],
    };

    let mut ans = Mint::new(1);
    for i in 0..n {
        ans = {
            let mut d = Mint::new(0);
            for j in 0..6 {
                d += ans * dices[i][j];
            };
            d
        };
    }
    println!("{}", ans);
}
