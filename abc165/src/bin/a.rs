use proconio::input;
//const MOD = 1_000_000_009

fn main() {
    input!{
        k: usize,
        a: usize,
        b: usize,
    }
    for i in 1..=1000{
        if a<=i*k && k*i<=b{
            println!("OK");
            return;
        }
    }
    println!("NG");
}
