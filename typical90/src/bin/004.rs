use proconio::input;
//const MOD = 1_000_000_007;

fn main() {
    input!{
        h: usize,
        w: usize,
        a: [[usize; w];h],
    }
    let mut ans = vec![vec![0;w];h];
    let mut sum_row = vec![0;w];
    let mut sum_colmun = vec![0;h];
    for y in 0..h{
        for x in 0..w{
            sum_colmun[y] += a[y][x];
            sum_row[x] += a[y][x];
        }
    }


    for y in 0..h{
        for x in 0..w{
            ans[y][x] = sum_row[x]+sum_colmun[y]-a[y][x];
        }
    }
    for i in ans{
        for j in i{
            print!("{} ", j);
        }
        println!()
    }
}
