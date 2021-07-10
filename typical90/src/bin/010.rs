use proconio::input;
//const MOD = 1_000_000_007;


fn main() {
    input!{
        n: usize,
        score: [[usize;2];n],
        q: usize,
        query: [[usize;2];q],
    }
    let mut csum1 = vec![0;n+1];
    let mut csum2 = vec![0;n+1];
    for i in 0..n{
        if score[i][0]==1{
            csum1[i+1] = score[i][1];
        }else{
            csum2[i+1] = score[i][1];
        }
    }
    
    for i in 1..n{
        csum1[i+1] += csum1[i];
        csum2[i+1] += csum2[i];
    }
    //println!("{:?}", csum1);
    //println!("{:?}", csum2);
    for i in 0..q{
        let l = query[i][0]-1;
        let r = query[i][1];
        println!("{} {}", csum1[r]-csum1[l], csum2[r]-csum2[l]);
    }
}
