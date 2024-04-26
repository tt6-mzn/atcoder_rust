use ac_library::Dsu;

fn main() {
    proconio::input! {
        n: usize,
        m: usize,
        edge: [(usize, usize); m]
    }
    let edge = edge
        .iter()
        .map(|(a, b)| (a - 1, b - 1))
        .collect::<Vec<(usize, usize)>>();

    // eprintln!("{:?}", edge);
    let mut deg = vec![0usize; n];
    for (a, b) in edge.iter() {
        deg[*a] += 1;
        deg[*b] += 1;
    }
    let mut uf = Dsu::new(n);
    for (a, b) in edge.iter() {
        uf.merge(*a, *b);
    }
    let groups = uf.groups();
    let mut ans = 0;
    for group in groups {
        let s = group.len();
        let mut flg = false;
        let mut num_edges = 0;
        for v in group {
            if deg[v] >= 2 {
                flg = true;
            }
            num_edges += deg[v];
        }
        num_edges = num_edges >> 1;
        if flg {
            ans += s * (s - 1) / 2 - num_edges;
        }
    }
    println!("{ans}");
}
