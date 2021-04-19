use proconio::input;
use proconio::marker;
//const mod: i64 = 10i64.pow(9i64) + 9;

fn main() {
    input!{
        s: marker::Chars,
        t: marker::Chars,
    }
    for i in 0..s.len(){
        if s[i]!=t[i]{println!("No"); return;}
    }
    println!("Yes");
}
