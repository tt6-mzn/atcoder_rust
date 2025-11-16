use proconio::input;

fn main() {
    input! {
        (h, w): (usize, usize),
    };
    if h == 1 || w == 1 {
        println!("{}", h * w);
    } else {
        println!("{}", (h / 2 + h % 2) * (w / 2 + w % 2));
    }
}
