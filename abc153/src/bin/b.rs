use proconio::input;

fn main() {
    input!{
        h: i64,
        n: usize,
        a: [i64; n]
    }
    let mut sum = 0;
    for i in a{
        sum+=i;
    }
    if h <= sum {println!("Yes");}
    else {println!("No");}
}
