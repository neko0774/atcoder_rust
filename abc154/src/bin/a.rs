use proconio::input;

fn main() {
    input!{
        s: String,
        t: String,
        a: i64,
        b: i64,
        u: String,
    }
    if u==s {println!("{} {}", a-1, b);}
    else {println!("{} {}", a, b-1);}
}
