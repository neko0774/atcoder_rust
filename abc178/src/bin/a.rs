use proconio::input;
//const MOD = 1_000_000_007;

fn main() {
    input!{
        x: usize,
    }
    match x{
        0 => println!("{}", 1),
        _ => println!("{}", 0),
    }
}
