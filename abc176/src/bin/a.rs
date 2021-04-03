use proconio::input;

fn main() {
    input!{
        n: usize,
        mut a: [u64; n],
    }
    let mut ans = 0;
    for i in 1..n{
        ans += a[i]-a[i-1];
        a[i] += a[i]-a[i-1];
    }
    println!("{}", ans);
}
