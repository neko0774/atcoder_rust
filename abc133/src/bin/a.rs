use proconio::input;

fn main() {
    input!{
        n: i64,
        a: i64,
        b: i64,
    }
    println!("{}", b.min(n*a));
}
