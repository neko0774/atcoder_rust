use proconio::input;
//const MOD = 1_000_000_007;

fn main() {
    input!{
        n: usize,
        k: usize,
        l: usize,
        a: [usize; n],
    }
    let mut lo = 0;
    let mut hi = n;
    let ans = while lo < hi{
        let mid = (lo+hi)/2;
        let mut len = 0;
        let mut cnt = 0;
        for i in &a{
            if len>mid{
                len = 0;
                cnt += 1;
            }else{
                len += i;
            }
        }
        if k < cnt{lo = mid+1;}
        else{hi = mid;}
        
        if lo>=hi{
            mid;
        }
    };
    println!("{}", ans);
}
