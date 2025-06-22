use proconio::input;

fn main() {
    input!{
        n: usize,
        q: usize,
    };
    let mut a = (0..n).map(|x| x + 1).collect::<Vec<_>>();
    let mut head = 0usize;
    for _ in 0..q {
        input! {
            t: usize,
        };
        match t {
            1 => {
                input! {
                    p: usize,
                    x: usize,
                }
                a[(head + p - 1) % n] = x;
            },
            2 => {
                input! {
                    p: usize,
                };
                println!("{}", a[(head + p - 1) % n]);
            },
            3 => {
                input! {
                    k: usize,
                }
                head = (head + k) % n;
            },
            _ => unreachable!()
        };
    }
}
