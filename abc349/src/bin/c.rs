use proconio::marker::Chars;

fn main() {
    proconio::input! {
        s: Chars,
        t: Chars
    }
    let n = if t[2] == 'X' { 2 } else { 3 };
    let mut i = 0;
    for ti in t.iter().take(n) {
        while s[i].to_ascii_uppercase() != *ti {
            i += 1;
            if i >= s.len() {
                println!("No");
                return;
            }
        }
        i += 1;
    }
    println!("Yes");
}
