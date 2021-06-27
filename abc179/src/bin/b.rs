use proconio::input;
//const MOD = 1_000_000_007;

fn main() {
    input!{
        n: usize,
    }
    let mut cnt = 0;
    let mut flag = false;
    for _ in 0..n{
        input!{
            i: usize,
            k: usize,
        }
        if i == k {
            cnt += 1;
        }else{
            cnt = 0;
        }
        if cnt==3{flag = true;}
        //println!("{}", flag);
    }
    if flag{println!("Yes");}
    else {println!("No");}
}
