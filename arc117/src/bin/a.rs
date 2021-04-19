use proconio::input;
//const mod: i64 = 10i64.pow(9i64) + 9;

fn main() {
    input!{
        a: i64,
        b: i64,
    }
    let mut ans = vec![0;(a+b) as usize];
    let num_a = 0;
    let num_b = 0;
    for i in 1..1000{
        for j in 1..1000{
            if a*(2*i+a-1) == b*(2*j+b-1){
                let num_a = i;
                let mun_b = j;
            }
        }
    }
    assert!(num_a==0, "false a");
    assert!(num_b==0, "false b");
    for i in 0..a{
       ans[i as usize] = num_a+i;
    }
    for j in 0..b{
        ans[(a+j) as usize] = -num_b-j;
    }
    for i in ans.iter(){
        print!("{} ", i);
    }
    println!()
}
