use proconio::input;

fn main() {
    input!{
        n: usize,
        a: [usize; n],
        b: [usize; n],
        c: [usize; n-1],
    }
    let mut ans=0;
    let mut bm: usize=100;
    for i in 0..n{
        ans += b[i];
        if bm+1==a[i]{ans+=c[bm-1];}
        bm = a[i];
    }
    println!("{}", ans);
}
