use proconio;
use itertools::Itertools;

fn main() {
	proconio::input! {
		n: usize,
		q: usize,
	};
	let mut boxes = vec![0usize; n];
	let mut ans = Vec::new();
	for _ in 0..q {
		proconio::input! {
			x: usize,
		};
		if x >= 1 {
			boxes[x - 1] += 1;
			ans.push(x);
		} else {
			let min_index = boxes.iter().position_min().unwrap();
			boxes[min_index] += 1;
			ans.push(min_index + 1);
		}
	}
	println!("{}", ans.iter().join(" "))
}
