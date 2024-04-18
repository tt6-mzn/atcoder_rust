use proconio::marker::Chars;

fn main() {
    proconio::input! {
        n: usize,
        s: Chars,
        c: [i64; n],
    }
    let s = s.iter().map(|c| match *c {
        '0' => 0,
        '1' => 1,
        _ => unreachable!()
    }).collect::<Vec<usize>>();
    
    // left[i][j] := 右端がjであるi文字の"半良い文字列"を，s[..i]から作るために必要な最小コスト
    let mut left = vec![vec![0i64; 2]; n + 1];
    // right[i][j] := 左端がjである(n-i)文字の"半良い文字列"を，s[i..]から作るために最小コスト
    let mut right = vec![vec![0i64; 2]; n + 1];
    
    left[0][0] = 0;
    left[0][1] = 0;
    for i in 1..n + 1 {
        for j in 0..2 {
            // s[i-1]をjにするときのコストを求める
            // s[i-2]は1-jになる
            if s[i - 1] == j {
                // s[i-1]は変える必要がない
                left[i][j] = left[i - 1][1 - j];   
            } else {
                // s[i-1]をコストc[i-1]かけて反転させる必要がある
                left[i][j] = left[i - 1][1 - j] + c[i-1];
            }
        }
    }

    right[n][0] = 0;
    right[n][1] = 0;
    for i in (0..n).rev() {
        for j in 0..2 {
            // s[i]をjにするときのコストを求める
            // s[i-1]は1-jになる
            if s[i] == j {
                // s[i]は変える必要がない
                right[i][j] = right[i + 1][1 - j];
            } else {
                // s[i]はコストc[i]をかけて反転させる必要がある
                right[i][j] = right[i + 1][1 - j] + c[i];
            }
        }
    }

    // eprintln!("{:?}", left);
    // eprintln!("{:?}", right);

    let mut ans = 1i64 << 60;
    for i in 0..n - 1 {
        // s[i] == s[i+1]になるように良い文字列を作る時のコスト
        for j in 0..2 {
            let mut cost = 0;
            if s[i] != j {
                cost += c[i];
            }
            if s[i + 1] != j {
                cost += c[i + 1];
            }
            cost += left[i][1 - j];
            if i + 2 <= n {
                cost += right[i + 2][1 - j];
            }
            ans = ans.min(cost);
        }
    }
    println!("{ans}");
}
