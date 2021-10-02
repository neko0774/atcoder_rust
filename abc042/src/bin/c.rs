use proconio::input;
use std::collections::HashSet;
//const MOD = 1_000_000_007;

fn main() {
    input!{
        n: usize,
        k: usize,
        mut d: [usize; k],
    }
    let d: HashSet<usize> = d.into_iter().collect();
    //println!("{:?}", d);
    for cnt in 0..10*n{
        let mut num = n+cnt;
        let ans = num;
        let mut num_list = HashSet::new();
        while num>0{
            num_list.insert(num%10);
            num /= 10;
        }
        //println!("{:?}", num_list);
        //println!("{:?}", d.intersection(&num_list));
        if   d.intersection(&num_list).count() == 0 {
            println!("{}", ans);
            return
        }
    }
}