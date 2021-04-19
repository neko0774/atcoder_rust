use proconio::input;
//const mod: i64 = 10i64.pow(9i64) + 9;

fn main() {
    input!{
        s:usize,
        w: usize,
    }
    if w>=s{println!("unsafe");}
    else {println!("safe");}
}
