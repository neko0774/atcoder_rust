use proconio::input;
//const mod: i64 = 10i64.pow(9i64) + 9;

fn main() {
    input!{
        x: usize,
    }
    let mut base = 100;
    for i in 1..100000{
        base *= 101;
        base /= 100;
        if base>=x{
            println!("{}", i);
            return;
        }
    }
}
