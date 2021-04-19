use proconio::input;
//const mod: i64 = 10i64.pow(9i64) + 9;

fn main() {
    input!{
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    }
    let tt = if a%d==0 {a/d} else {a/d+1};
    let at = if c%b==0 {c/b} else {c/b+1};
    if tt>=at {println!("Yes");}
    else {println!("No");}
}
