use proconio::input;
use ac_library::ModInt1000000007 as Mint;

fn main() {
    input! {
        (l, r): (u64, u64),
    };
    let r = r + 1;

    let mut a = Vec::new();
    a.push(l);
    for e in (l.ilog10() + 1)..=((r - 1).ilog10()) {
        a.push(10u64.pow(e));
    }
    a.push(r);
    
    // println!("{:?}", a);

    let inv2 = Mint::new(2).inv();
    let mut ans = Mint::new(0);
    for w in a.windows(2) {
        let dx = Mint::new(w[0].ilog10() + 1);
        let x = Mint::new(w[0]);
        let y = Mint::new(w[1] - 1);
        let s = dx * (x + y) * (y - x + 1) * inv2;
        ans += s;
    }
    println!("{}", ans.val());
}
