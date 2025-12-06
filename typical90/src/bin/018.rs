use libm::atan2;
use proconio::input;
use std::f64::consts::PI;

fn main() {
    input! {
        t: f64,
        (l, x, y): (f64, f64, f64),
        q: usize,
    };
    // 高橋直大像の座標
    let s = (x, y, 0.0);

    for _ in 0..q {
        input! {
            e: f64,
        };
        // 観覧車の回転角
        let theta = 2.0 * PI * e / t;
        // 観覧車の座標
        let k = (
            0.0,
            -(l/2.0) * theta.sin(),
            l/2.0 - l/2.0 * theta.cos(),
        );
        // 高橋直大像との相対座標のz成分
        let skz = (s.2 - k.2).abs();
        // 高橋直大像との相対座標の水平面への射影の大きさ
        let psk = {
            let mut ret = 0.0;
            ret += (s.0 - k.0).powf(2.0);
            ret += (s.1 - k.1).powf(2.0);
            ret.sqrt()
        };
        let ans = atan2(skz, psk).to_degrees();
        println!("{}", ans);
    }
}
