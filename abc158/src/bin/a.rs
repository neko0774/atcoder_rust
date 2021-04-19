use proconio::input;
use proconio::marker::Chars;
//const mod: i64 = 10i64.pow(9i64) + 9;

fn main() {
    input!{
        s: Chars,
    }
    if s[0]==s[1]&&s[1]==s[2]{println!("No");}
    else {println!("Yes");}
}
