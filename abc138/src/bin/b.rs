use proconio::input;

fn main() {
    input!{
        n: usize,
        a: [f64; n],
    }
    let mut ans: f64 = 0.0;
    for i in a{
        ans += 1.0/i;
    }
    println!("{}", 1.0/ans);
}
