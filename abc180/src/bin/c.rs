use proconio::input;
//const MOD = 1_000_000_007;

fn main() {
    input!{
        n: usize,
    }
    let mut i = 1;
    let mut front = Vec::new();
    let mut back = Vec::new();
    while i*i <= n{
        if n%i==0{
            if n/i==i{front.push(i);}
            else {
                front.push(i);
                back.push(n/i);
            }
        }
        i += 1;
    }
    for i in front.iter(){
        println!("{}", i);
    }
    for i in back.sort_by_key(k, Reverse(OK)).iter(){
        println!("{}", i);
    }
 }
