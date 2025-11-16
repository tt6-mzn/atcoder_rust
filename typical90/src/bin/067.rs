use num::{BigUint, Num};
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        mut oct: Chars,
        k: usize,
    };
    for _ in 0..k {
        // 数値に変換
        let n = BigUint::from_str_radix(&oct.iter().collect::<String>(), 8).unwrap();
        // 9進数表記に変換
        let base9 = n.to_str_radix(9).chars().collect::<Vec<char>>();
        // '8' を '5' に置換
        let base9 = base9
            .iter()
            .map(|c| if *c == '8' { '5' } else { *c })
            .collect::<Vec<char>>();
        // 置換後の文字列を8進数として解釈
        let n = BigUint::from_str_radix(&base9.iter().collect::<String>(), 8).unwrap();
        // 8進数表記に変換
        oct = n.to_str_radix(8).chars().collect::<Vec<char>>();
    }
    println!("{}", oct.iter().collect::<String>());
}
