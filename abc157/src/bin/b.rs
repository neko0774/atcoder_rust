use proconio::input;
//const mod: i64 = 10i64.pow(9i64) + 9;

fn main() {
    input!{
        a: [[usize; 3]; 3],
        n: usize,
        b: [usize; n],
    }
    let mut c = [[false; 3]; 3];
    let mut flag = false;
    for k in 0..n{
        for i in 0..3{
            for j in 0..3{
                if a[i][j] == b[k]{c[i][j]=true;}
            }
        }
        for l in 0..3{
            if c[l][0] && c[l][1] && c[l][2] || c[0][l] && c[1][l] && c[2][l]{flag=true;}
        }
        if c[0][0] && c[1][1] && c[2][2] || c[0][2] && c[1][1] && c[2][0]{flag=true;}
    }
    println!("{}", if flag {"Yes"} else {"No"});
}
