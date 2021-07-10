use proconio::input;
use std::collections::HashSet;
//const MOD = 1_000_000_007;

fn main() {
    input!{
        n: usize,
    }
    let mut ans_full = HashSet::new();
    for i in 0..(1 << n){
        let bit = (0..n)
        .map(|x| if i & (1<<x)>0 {1} else {0});
        let mut ans = String::new();        
        for j in bit{
            //ここに処理を書く            
            //print!("{} ", j);
            if j==0{ans += "("}
            else {ans += ")"}
        }
        let mut f=0;
        let mut b=0;
        for k in ans.chars(){
            if &k.to_string()=="("{f+=1;}
            else{b += 1;}

            if b>f{break;}
        }
        if f==b{ans_full.insert(ans);}
    }
    for j in ans_full.iter(){
        println!("{}", j);
    }
}
