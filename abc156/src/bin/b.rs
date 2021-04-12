use proconio::input;

fn main() {
    input!{
        n: i64,
        k: i64,
    }
    let mut cnt=1;
    let mut num=k;
    while num<=n{
        cnt += 1;
        num *= k;
    }
    println!("{}", cnt);
}
