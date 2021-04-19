use proconio::input;
//const mod: i64 = 10i64.pow(9i64) + 9;

fn main() {
    input!{
        k: usize,
        n: usize,
        a: [usize; n],
    }
    let mut dis = a[0]+k-a[n-1];
    for i in 1..n{
        dis = dis.max(a[i]-a[i-1]);
    }
    println!("{}", k-dis);
}
