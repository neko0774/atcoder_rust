use proconio::input;
//const mod: i64 = 10i64.pow(9i64) + 9;

fn main() {
    input!{
        l: f64,
    }
    println!("{}", l*l*l/27.0);
}
