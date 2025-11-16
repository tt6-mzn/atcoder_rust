use proconio::input;

fn main() {
    input! {
        (n, m): (usize, usize),
    };
    let graph = {
        input! {
            edges: [(i32, i32); m],
        }
        let mut ret = vec![vec![]; n];
        for (a, b) in edges {
            let (i, j) = ((a - 1) as usize, (b - 1) as usize);
            ret[i.min(j)].push(i.max(j));
        }
        ret
    };
    let mut num_lower = vec![0; n];
    num_lower[0] = 0;
    for i in 0..n {
        for j in &graph[i] {
            num_lower[*j] += 1;
        }
    }
    let mut ans = 0;
    for i in 0..n {
        if num_lower[i] == 1 {
            ans += 1;
        }
    }
    println!("{}", ans);
}
