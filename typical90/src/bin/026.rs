use std::collections::VecDeque;
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n - 1],
    };

    let mut graph = vec![vec![]; n];
    for (a, b) in ab {
        let a = a - 1;
        let b = b - 1;
        graph[a].push(b);
        graph[b].push(a);
    }
    let graph = graph;

    let mut parity = vec![false; n];
    let mut visited = vec![false; n];
    let mut que = VecDeque::new();

    que.push_back(0);
    while let Some(v) = que.pop_front() {
        if visited[v] {
            continue;
        }

        visited[v] = true;
        for &nv in &graph[v] {
            parity[nv] = !parity[v];
            que.push_back(nv);
        }
    }

    let mut even = vec![];
    let mut odd = vec![];
    for v in 0..n {
        if parity[v] {
            even.push(v);
        } else {
            odd.push(v);
        }
    }
    let ans = if even.len() >= odd.len() {
        even
    } else {
        odd
    };
    println!("{}", ans.iter().take(n / 2).map(|v| v + 1).join(" "));
}