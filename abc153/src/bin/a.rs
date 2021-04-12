use proconio::input;

fn main() {
    input!{
        h: i64,
        a: i64,
    }
    match h%a{
        0 => println!("{}", h/a),
        _ => println!("{}", h/a+1),
    }
}
