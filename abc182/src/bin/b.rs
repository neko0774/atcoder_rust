use proconio::input;

fn main() {
    input!{
        n: usize,
        a: [usize; n],
    }
    let mut ans = 0;
    let mut cnt = 0; 
    for i in 2..=1000{
        let mut c=0;
        for j in &a{
            if j%i==0{c+=1;}
        }
        if c>=cnt{
            ans=i;
            cnt = c;
        }
    }
    println!("{}", ans);
}
