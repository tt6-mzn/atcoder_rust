use regex::Regex;

fn main() {
    proconio::input! {
        s: String
    }
    let re = Regex::new("^<=+>$").unwrap();
    let caps = re.captures(s.as_str());
    // eprintln!("{:?}", caps);
    if let Some(_) = caps {
        println!("Yes");
    } else {
        println!("No");
    }
}
