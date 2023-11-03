fn is_326_like_number(mut n: impl Iterator<Item = char>) -> bool {
    let x = n.next().unwrap().to_digit(10).unwrap(); 
    let y = n.next().unwrap().to_digit(10).unwrap(); 
    let z = n.next().unwrap().to_digit(10).unwrap(); 
    if x * y == z {
        return true;
    }
    return false;
}

fn main() {
    proconio::input! {
        mut n: i32,
    }
    while !is_326_like_number(n.to_string().chars()) {
        n += 1;
    }
    println!("{n}");
}
