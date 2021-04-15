use proconio::input;
//const mod: i64 = 10i64.pow(9i64) + 9;

fn main() {
    input!{
        n: usize,
    }
    let fb = n/15*(30+(n/15-1)*15)/2;
    let f = n/3*(6+(n/3-1)*3)/2;
    let b = n/5*(10+(n/5-1)*5)/2;

    let ans = n*(n+1)/2-f-b+fb;
    //println!("{} {} {} {}", n*(n+1)/2, fb, b, f);
    println!("{}", ans);
}
