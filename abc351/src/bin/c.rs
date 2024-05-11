use std::collections::VecDeque;

fn main() {
    proconio::input! {
        n: usize,
        a: [i64; n],
    }
    let mut st = VecDeque::new();
    for ai in a {
        st.push_back(ai);
        loop {
            if st.len() <= 1 {
                break;
            }
            let right = st.pop_back().unwrap();
            let left = st.pop_back().unwrap();
            if left != right {
                st.push_back(left);
                st.push_back(right);
                break;
            }
            st.push_back(left + 1);
        }
    }
    println!("{}", st.len());
}
