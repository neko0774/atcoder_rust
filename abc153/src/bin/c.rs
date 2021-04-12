use proconio::input;

fn main() {
    input!{
        n: usize,
        k: i64,
        mut h: [i64; n],
    }
    h.sort();
    let mut ans=0;
    for i in 0..0.max(n as i64-k) as usize{
        ans += h[i];
    }
    println!("{}", ans);
}
