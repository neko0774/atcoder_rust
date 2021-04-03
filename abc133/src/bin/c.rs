use proconio::input;

fn main() {
    input!{
        l: i64,
        r: i64,
    }
    let mut ans=2020;
    let lim = r.min(l+2020)+1;
    for i in l..lim{
        for j in i+1..lim{
            ans = ans.min((i*j)%2019);
        }
    }
    println!("{}", ans);
}
