use proconio::input;
//const MOD = 1_000_000_007;

fn main() {
    input!{
        n: usize,
        x: [i128;n],
    }
    let mut md = 0;
    let mut ed = 0;
    let mut td = 0;
    for i in x.iter(){
        md += i.abs();
        ed += i*i;
        td = td.max(*i);
    }
    println!("{}", md);
    println!("{}", f64::sqrt(ed as f64));
    println!("{}", td);
}
