use itertools::Itertools;
use proconio::marker::Chars;

fn main() {
    proconio::input! {
        s: Chars
    }
    if s.iter().join("") == "ABC316".to_string() {
        println!("No");
        return;
    }
    if s.iter().join("") == "ABC000".to_string() {
        println!("No");
        return;
    }
    match s[3] {
        '0' | '1' | '2' => println!("Yes"),
        '3' => match s[4] {
            '0' | '1' | '2' | '3' | '4' => println!("Yes"),
            _ => println!("No"),
        },
        _ => println!("No"),
    }
}
