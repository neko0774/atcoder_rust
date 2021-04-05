use proconio::input;

fn main() {
    input!{
        n: usize,
        b: [usize; n-1],
    }
    let mut ans = b[0]+b[n-2];
    for i in 0..n-2{
        ans += b[i].min(b[i+1]);
    }
    println!("{}", ans);
}
