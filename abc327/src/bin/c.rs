fn is_ok(v: &Vec<i32>) -> bool {
    for i in 1..=9 {
        if !v.contains(&i) {
            return false;
        }
    }
    true
}

fn main() {
    proconio::input! {
        a: [[i32; 9]; 9],
    }
    for a_row in &a {
        if !is_ok(a_row) {
            println!("No");
            return;
        }
    }
    for j in 0..9 {
        let mut a_col = Vec::new();
        for i in 0..9 {
            a_col.push(a[i][j]);
        }
        if !is_ok(&a_col) {
            println!("No");
            return;
        }
    }

    for i in (0..9).step_by(3) {
        for j in (0..9).step_by(3) {
            let mut a_crop = Vec::new();
            for k in 0..3 {
                for l in 0..3 {
                    a_crop.push(a[i + k][j + l]);
                }
            }
            if !is_ok(&a_crop) {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}
