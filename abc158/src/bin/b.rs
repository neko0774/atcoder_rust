use proconio::input;
//const mod: i64 = 10i64.pow(9i64) + 9;

fn main() {
    input!{
        n: usize,
        a: usize,
        b: usize,
    }
    let ans = a*n/(a+b) + std::cmp::min(n%(a+b), a);
    println!("{}", ans);
}
