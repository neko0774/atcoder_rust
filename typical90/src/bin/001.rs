use proconio::input;
//const MOD = 1_000_000_007;

fn main() {
    input!{
        n: usize,
        l: usize,
        k: usize,
        mut a: [usize; n],
    }
    //b search
    let mut lo = 0;
    let mut hi = l;
    let mut ans = l;
    while lo < hi{
        let mid = (lo+hi)/2;
        let mut len_c = 0;
        let mut cnt = 0;
        ans = l;
        for i in 0..n-1{
            if a[i]-len_c>=mid{
                cnt += 1;
                ans = ans.min(a[i]-len_c);
                len_c = a[i];
            }
        }
        ans = ans.min(l-len_c);
        println!("{} {} {} {}", lo, hi, cnt, ans);
        if cnt<=k{hi=mid;}
        else{lo=mid+1;}
    }
    println!("{}", ans);

    

}
