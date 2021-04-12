use proconio::input;
use proconio::marker::Chars;

fn main() {
    input!{
        s: Chars,
    }
    let mut cnt = 0;
    let mut c = [0;3];
    for i in &s{
        let n = *i as usize - 48;
        cnt += n%3;
        c[n%3] += 1;
    }
    //println!("{:?}", c);

    if cnt%3==0{
        println!("{}", 0);
    }else if ((cnt%3==1&&c[1]>=1) || (cnt%3==2&&c[2]>=1))&&s.len()!=1{
        println!("{}", 1);
    }else if ((cnt%3==2&&c[1]>=2) || (cnt%3==1&&c[2]>=2)) && s.len()!=2{
        println!("{}", 2);
    }else {
        println!("{}", -1);
    }
}
