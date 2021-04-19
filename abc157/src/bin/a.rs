use proconio::input;
//const MOD: i64 = 10i64.pow(9u32) + 9;

fn main() {
    input!{
        n: i64,
    }
    match n%2{
        0 => println!("{}", n/2),
        _ => println!("{}", n/2+1),
    }
}
