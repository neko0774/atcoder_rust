use proconio::input;

fn main() {
    input!{
        a: i64,
        b: i64,
    }
    let mut rep = 0;
    if a>b{rep=a;}
    else {rep=b;}

    for _ in 0..rep{
        print!("{}", a+b-rep);
    }
    println!()
}
