use proconio::input;

fn main() {
    input!{
        h: usize,
        n: usize,
        ab: [(usize, usize);n],
    }
    let mut dp = vec![10_usize.pow(10); h+1];
    dp[0] = 0;
    //もらうdp
    //O(10^7)
    for i in 0..=h{
        for j in 0..n{
            let ma = ab[j].0;
            let mc = ab[j].1;
            if i-ma>=0 && dp[i]>dp[i-ma] { dp[i]=dp[i-ma]+mc;}
        }
    }
    println!("{}", dp[n]);
}
