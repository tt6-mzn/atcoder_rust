use std::collections::BTreeSet;

fn main() {
    proconio::input! {
        n: usize,
        k: usize,
        p: [usize; n]
    }
    if k == 1 {
        println!("0");
        return;
    }
    
    let index = {
        let mut ret = vec![0; n + 1];
        for (i, pi) in p.iter().enumerate() {
            ret[*pi] = i;
        }
        ret
    };
    // 1..kを作る時の答えを求める
    let mut btree = BTreeSet::new();
    for ai in 1..(k + 1) {
        btree.insert(index[ai]);
    }
    let mut ans = btree.last().unwrap() - btree.first().unwrap();
    // eprintln!("{} {}", btree.last().unwrap(), btree.first().unwrap());
    btree.remove(&index[1]);
    for ak in (k + 1)..=n {
        // (ak - k + 1)..akを作る時の答えを求める
        btree.insert(index[ak]);
        ans = ans.min(btree.last().unwrap() - btree.first().unwrap());
        // eprintln!("{} {}", btree.last().unwrap(), btree.first().unwrap());
        btree.remove(&index[ak - k + 1]);
    }
    println!("{ans}");
}
