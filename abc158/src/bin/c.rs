use proconio::input;
//const mod: i64 = 10i64.pow(9i64) + 9;

fn main() {
    input!{
        a: usize,
        b: usize,
    }
    for i in 1..2000{
        if i*8/100==a && i*10/100==b{
            println!("{}", i);
            return;
        }
    }
    println!("-1");
}
