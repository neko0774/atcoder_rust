use proconio::input;

fn main() {
    input! {
        n: i32,
        mut a: [i64; n],
    }
    let mut ans = 0;
    for i in 0..a.len()-1 {
        if a[i+1] < a[i] {
            ans+= a[i] - a[i+1];
            a[i+1] = a[i];
        }
    }

    println!("{}", ans);
}
