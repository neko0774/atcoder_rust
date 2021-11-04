use proconio::input;
//const MOD = 1_000_000_007;

fn main() {
    input!{
        x: String,
    }
    for i in x.chars() {
        if i=='.'{
            return;
        }else{
            print!("{}", i);
        }
    }
}
