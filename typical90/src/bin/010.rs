use proconio::input;

fn main() {
    input! {
        n: usize,
        cp: [(usize, i32); n],
    };
    let mut acc1 = vec![0; n + 1];
    let mut acc2 = vec![0; n + 1];
    for (i, &(c, p)) in cp.iter().enumerate() {
        match c {
            1 => acc1[i + 1] = p,
            2 => acc2[i + 1] = p,
            _ => unreachable!(),
        };
    }
    for i in 1..=n {
        acc1[i] += acc1[i - 1];
        acc2[i] += acc2[i - 1];
    }

    input! {
        q: usize,
    };
    for _ in 0..q {
        input! {
            (l, r): (usize, usize),
        };
        let ans1 = acc1[r] - acc1[l - 1];
        let ans2 = acc2[r] - acc2[l - 1];
        println!("{} {}", ans1, ans2);
    }
}
