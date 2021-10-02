use proconio::input;
//const MOD = 1_000_000_007;

fn main() {
    input!{
        n: usize,
        h: [i64;n],
    }
    let mut dp = vec![0;n];
    dp[1] = (h[0]-h[1]).abs();
    for i in 2..n{
        dp[i] = std::cmp::min(dp[i-2]+(h[i]-h[i-2]).abs(), dp[i-1]+(h[i]-h[i-1]).abs());
        //println!("{}", dp[i]);
    }
    println!("{}", dp[n-1]);
}
