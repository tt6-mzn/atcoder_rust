use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [usize; n],
    }
    let mut zp = 0;
    for _ in 0..q {
        input! {
            t: usize,
            x: usize,
            y: usize,
        }
        match t {
            1 => {
                let x = x - 1;
                let y = y - 1;
                a.swap((zp + x) % n, (zp + y) % n);
            }
            2 => {
                zp = (zp + n - 1) % n;
            }
            3 => {
                let x = x - 1;
                println!("{}", a[(zp + x) % n]);
            }
            _ => unreachable!(),
        }
    }
}
