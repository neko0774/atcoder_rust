use proconio::input;
//const MOD = 1_000_000_007;

fn main() {
    input!{
        n: String,
    }
    let s = n.chars().count();
    if s==1{
        println!("{}", 0);
    }else if s%2==1{
        println!("{}", "9".repeat(s/2));
    }else{
        let left = n.parse::<usize>().unwrap()/10usize.pow((s/2) as u32);
        let right = n.parse::<usize>().unwrap()%10usize.pow((s/2) as u32);
        if right>=left{
            println!("{}", left);
        }else{
            println!("{}", left-1);
        }
    }
}
