use proconio::input;
use num::integer::gcd;
//const mod: i64 = 10i64.pow(9i64) + 9;

fn main() {
    input!{
        k: usize,
    }
    let mut ans = 0;
    for i in 1..=k{
        for j in 1..=k{
            for k in 1..=k{
                ans += gcd(gcd(i,j),k);
            }
        }
    }
    println!("{}", ans);
}
