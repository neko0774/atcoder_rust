use proconio::input;
use std::collections::HashSet;

fn main() {
    input!{
        n: usize,
    }
    let mut set = HashSet::new();
    for _ in 0..n{
        input!{s: String,}
        set.insert(s);
    }
    println!("{}", set.len());
}
