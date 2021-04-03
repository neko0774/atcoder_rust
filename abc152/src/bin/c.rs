use proconio::input;

fn main() {
    input!{
        n: usize,
        p: [i64; n],
    }
    let mut ans=1;
    let mut min=p[0];
    for i in 1..n {
        if p[i]<min{
            ans += 1;
            min = p[i];
        }
    }
    println!("{}", ans);
}
