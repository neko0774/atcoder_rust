use proconio::input;
//const MOD = 1_000_000_007;

fn main() {
    input!{
        n: usize,
    }
    let mut ans = 0;
    for i in 0..n{
        input!{
            a: usize,
            b: usize,
        }
        ans += (b+a)*(b-a+1)/2;
    }
    println!("{}", ans);
}
