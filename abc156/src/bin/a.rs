use proconio::input;

fn main() {
    input!{
        n: i64,
        r: i64,
    }
    if n>=10{println!("{}", r);}
    else {println!("{}", r+100*(10-n));}    
}
