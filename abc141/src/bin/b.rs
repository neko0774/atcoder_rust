use proconio::input;

fn main() {
    input!{
        s: String,
    }
    let mut turn = true;
    for i in s.chars(){
        //println!("{}", turn);
        if turn && i=='L'{
            println!("No");
            return;
        }
        else if !turn && i=='R'{
            println!("No");
            return;
        }
        turn = !turn;
    }
    println!("Yes");
}
