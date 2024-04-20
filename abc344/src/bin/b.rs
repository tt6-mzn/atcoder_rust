fn main() {
    let mut ans = Vec::new();
    loop {
        proconio::input! {
            ai: i32
        }
        ans.push(ai);
        if ai == 0 {
            break;
        }
    }
    for ai in ans.iter().rev() {
        println!("{ai}");
    }
}
