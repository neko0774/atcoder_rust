use proconio::input;
use proconio::marker::Chars;
//const mod: i64 = 10i64.pow(9i64) + 9;

fn main() {
    input!{
        s: Chars,
    }
    let l = s.len();
    for i in 0..l/2 {
        if s[i] != s[l-i-1]{
            println!("No");
            //println!("{} {}", s[i], s[l-i-1]);
            return;
        }
    }
    for i in 0..s.len()/4{
        if s[i]!=s[l/2-1-i] || s[l/2+1+i]!=s[l-i-1]{
            println!("No");
            //println!("{} {}", s[i] == s[l/2-1-i], s[l/2+1+i] == s[l-i-1]);
            return;
        }
    }
    println!("Yes");
}
