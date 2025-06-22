use proconio::input;
use std::collections::{HashMap, VecDeque};

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(usize, usize, u32); m],
    };
    let mut graph = vec![vec![]; n];
    for (a, b, w) in edges {
        graph[a - 1].push((b - 1, w));
    }

    let mut graph_ext = HashMap::new();
    let mut que = VecDeque::new();
    que.push_back((0, 0));

    while !que.is_empty() {
        let (v, w) = que.pop_front().unwrap();
        if graph_ext.contains_key(&(v, w)) {
            continue;
        }
        graph_ext.insert((v, w), true);
        for (v_next, w_next) in graph[v].iter() {
            que.push_back((*v_next, w ^ w_next));
        }
    }
    let mut ans = None;
    for w in 0..(1 << 10) {
        if graph_ext.contains_key(&(n - 1, w)) {
            ans = Some(w);
            break;
        }
    }
    if let Some(w) = ans {
        println!("{}", w);
    } else {
        println!("-1");
    }
}