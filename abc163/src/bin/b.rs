use proconio::input;
//const mod: i64 = 10i64.pow(9i64) + 9;

fn main() {
    input!{
        n: usize,
        m: usize,
        a: [usize; m],
    }
    let sum = a.iter().sum();
    if n>=sum{println!("{}", n-sum);}
    else {println!("-1");}
    
}

