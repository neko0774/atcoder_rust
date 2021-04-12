use proconio::input;

fn main() {
    input!{
        mut h: i64,
    }
    let mut cnt=1;
    while h>1{
        h /= 2;
        cnt += 1;
    }
    let mut ans: u128=1;
    for _ in 0..cnt{
        ans *= 2;
    }
    println!("{}",ans-1);
}
