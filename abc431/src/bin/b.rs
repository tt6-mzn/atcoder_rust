use proconio::input;

fn main() {
    input! {
        mut x: i32,
        n: usize,
        w: [i32; n],
        q: usize,
        p: [usize; q],
    };
    let mut is_installed = vec![false; n];
    for pi in p {
        let pi = pi - 1;
        if is_installed[pi] {
            x -= w[pi];
            is_installed[pi] = false;
        } else {
            x += w[pi];
            is_installed[pi] = true;
        }
        println!("{}", x);
    }
}
