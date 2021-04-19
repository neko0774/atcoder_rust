use proconio::input;
//const mod: i64 = 10i64.pow(9i64) + 9;

fn main() {
    input!{
        a: i64,
        b: i64,
        c: i64,
        k: i64,
    }
    if k<=a+b{println!("{}", a);}
    else {println!("{}", a-c+(a+b+c-k));}
}
