use proconio::input;
//const MOD = 1_000_000_007;

fn main() {
    input!{
        n: usize,
        l: usize,
        mut s: [String; n],
    }
    s.sort();
    for i in s{
        print!("{}", i);
    }
    
}
