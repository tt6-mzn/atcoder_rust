fn main() {
    let s = "wbwbwwbwbwbw".chars().collect::<Vec<char>>();
    proconio::input! {
        w: i32,
        b: i32
    }
    for i in 0..s.len() {
        let mut nw = 0;
        let mut nb = 0;
        for j in 0..(w + b) as usize {
            match s[(i + j) % s.len()] {
                'w' => nw += 1,
                'b' => nb += 1,
                _ => unreachable!()
            }
        }
        if nw == w && nb == b {
            println!("Yes");
            return;
        }
    }
    println!("No");
}