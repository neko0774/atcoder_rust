use proconio::input;
//const MOD = 1_000_000_007;

fn main() {
    input!{
        a: usize,
        b: usize,
        c: usize,
    }
    if (a==5&&b==5&&c==7)||(a==5&&b==7&&c==5)||(a==7&&b==5&&c==5){
        println!("YES")
    }else{
        println!("NO")
    }
}
