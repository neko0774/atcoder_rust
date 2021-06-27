use proconio::input;
//const MOD = 1_000_000_007;

fn main() {
    input!{
        sx: f64,
        sy: f64,
        gx: f64,
        gy: f64,
    }
    let g = (sy+gy)/(gx-sx);
    let ans = sx+sy/g;
    println!("{}", ans);
}
