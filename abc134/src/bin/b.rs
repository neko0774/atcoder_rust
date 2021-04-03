use proconio::input;

fn main() {
    input!{
        n: i64,
        d: i64,
    }
    match n%(2*d+1){
        0 => println!("{}", n/(2*d+1)),
        _ => println!("{}", n/(2*d+1)+1),
    }
}
