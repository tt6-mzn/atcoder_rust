fn main() {
    proconio::input! {
        (h, w, m): (usize, usize, usize),
        query: [(i32, usize, usize); m],
    }
    let mut count_row = 0;
    let mut count_column = 0;

    let mut filled_row = vec![false; h];
    let mut filled_column = vec![false; w];

    let mut num_color = vec![0; 2*100000 + 1];
    num_color[0] = h * w;
    
    for (t, a, x) in query.iter().rev() {
        if *t == 1 {
            if filled_row[a - 1] {
                continue;
            }
            num_color[*x] += w - count_column;
            num_color[0] -= w - count_column;
            count_row += 1;
            filled_row[a - 1] = true;
        }
        if *t == 2 {
            if filled_column[a - 1] {
                continue;
            }
            num_color[*x] += h - count_row;
            num_color[0] -= h - count_row;
            count_column += 1;
            filled_column[a - 1] = true;
        }
    }
    let mut ans = Vec::new();
    for (c, nc) in num_color.iter().enumerate() {
        if *nc > 0 {
            ans.push((c, nc));
        }
    }
    println!("{}", ans.len());
    for (c, nc) in ans {
        println!("{c} {nc}");
    }
}
