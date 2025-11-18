use proconio::input;
use ac_library::ModInt1000000007 as Mint;

fn main() {
    input! {
        (n, k): (u64, u32),
    };

    let mut ans = Mint::new(1);
    if n == 1 {
        println!("{}", k);
        return;
    }
    if n == 2 && k == 1 {
        println!("0");
        return;
    }
    if n == 2 && k == 2 {
        ans *= k;
        ans *= k - 1;
        println!("{}", ans.val());
        return;
    }
    if n >= 3 && k <= 2 {
        println!("0");
        return;
    }
    ans *= k;
    ans *= k - 1;
    ans *= Mint::new(k - 2).pow(n - 2);
    println!("{}", ans.val());
}
