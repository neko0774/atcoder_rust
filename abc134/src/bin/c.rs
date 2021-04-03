use proconio::input;


fn main() {
    input!{
        n: usize,
        a: [usize; n],
    }
    let mut max = 0;
    let mut smax = 0;
    let mut index=0;
    for i in 0..n{
        if a[i]>=max{
            smax=max;
            max=a[i];
            index=i;
        }
        else{
            smax=smax.max(a[i]);
        }
    }
    for j in 0..n{
        println!("{}", if j==index{smax} else {max})
    }
    
}
