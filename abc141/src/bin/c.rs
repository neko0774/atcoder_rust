use proconio::{fastout, input};

#[fastout]
fn main() {
    input!{
        n: usize,
        k: usize,
        q: usize,
        a: [usize; q],
    }
    let mut score = vec![0; n];
    for i in a{
        score[i-1] += 1;
    }
    //println!("{:?}", score);
    for j in score{
        if k as i64-q as i64+j as i64>0{println!("Yes");}
        else {println!("No");}
    }
}
