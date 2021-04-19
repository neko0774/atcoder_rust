use proconio::{input, fastout};
use std::*;
//const mod: i64 = 10i64.pow(9i64) + 9;

#[fastout]
fn main() {
    input!{
        n: i64,
        x: i64,
        y: i64,
    }
    let mut ans = vec![0;n as usize-1];
    for i in 1..=n{
        for j in i+1..=n{
            //println!("{} {}", i, j);
            let dis = cmp::min(j-i-1, (y-j).abs()+(x-i).abs());
            ans[dis as usize] += 1;
    }}
    for i in ans.iter(){
        println!("{}", i);
    }

}
