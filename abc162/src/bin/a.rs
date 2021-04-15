use proconio::input;
//const mod: i64 = 10i64.pow(9i64) + 9;

fn main() {
    input!{
        n: String,
    }
    for i in n.chars(){
        if i=='7'{println!("Yes"); return;}
    }
    println!("No");
}
