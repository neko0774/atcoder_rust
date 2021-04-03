use proconio::input;

fn main() {
    input!{
        n: usize,
        d: usize,
        x: [[i64;d];n],
    }
    let mut ans=0;
    for i in 0..n{
        for j in i+1..n{
            let mut dis = 0;
            for k in 0..d{
                dis += (x[i][k]-x[j][k]).pow(2);
            }
            if dis==((dis as f64).sqrt() as i64).pow(2){ans+=1;}
        }
    }
    println!("{}", ans);
}
