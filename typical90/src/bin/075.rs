use proconio::input;

fn main() {
    input! {
        mut n: usize,
    };

    let mut c = {
        let mut ret: usize = 0;
        let mut tn = n;
        for i in 2.. {
            if i * i > n {
                break;
            }
            while tn % i == 0 {
                tn /= i;
                ret += 1;
            }
        }
        if tn > 1 {
            ret += 1;
        }
        ret
    };

    let ans = if c == 1 { 0 } else { (c - 1).ilog2() + 1 };
    println!("{}", ans);
}
