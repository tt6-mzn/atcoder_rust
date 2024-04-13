fn main() {
    proconio::input! {
         n: usize,
         k: i32,
         a: [i32; n]
    }
    let mut ans = Vec::new();
    for ai in a {
        if ai % k == 0 {
            ans.push(ai / k);
        }
    }
    // ansの内容を空白区切りで出力
    println!(
        "{}",
        ans.iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
