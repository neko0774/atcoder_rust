use proconio::input;
//const MOD = 1_000_000_007;
fn main() {
    input!{
        n: usize,
        grids: [[f64; 2];n],
    }
    //brute force 10^6
    for i in 0..n-2{
        for j in i+1..n-1{
            for k in j+1..n{
                if equation(grids[i][0], grids[i][1], grids[j][0], grids[j][1], grids[k][0], grids[k][1]){
                    println!("Yes");
                    return;
                }
            }
        }
    }
    println!("No");

    fn equation(x1:f64,y1:f64,x2:f64,y2:f64,x3:f64,y3:f64) -> bool{
        // x = k
        if x1==x2||x2==x3{
            if x1==x3&&x2==x3{return true;}
            else {return false;}
        }//y = k
        else if y1==y2{
            if y1==y3{return true;}
            else {return false;}
        }
        else if (y2-y1)/(x2-x1)==(y3-y2)/(x3-x2){
            return true;
        }
        else {return false;}
        
    }
}
