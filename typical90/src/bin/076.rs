use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };
    let sum = a.iter().sum();
    let mut acc = vec![vec![0], a.clone(), a].concat();
    for i in 1..acc.len() {
        acc[i] += acc[i - 1];
    }

    for start in 0..n {
        let mut left = 0;
        let mut right = n;
        while right - left > 1 {
            let mid = left + (right - left) / 2;
            // [start:start+mid]
            let s = acc[start + mid] - acc[start];
            if s * 10 <= sum {
                left = mid;
            } else {
                right = mid;
            }
        }
        let s = acc[start + left] - acc[start];
        if s * 10 == sum {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
