use proconio::input;
//const mod: i64 = 10i64.pow(9i64) + 9;

fn main() {
    input!{
        n: usize,
        m: usize,
        k: usize,
        //f: [[usize; 2]; m],
        //b: [[usize; 2]; k],
    }
    let mut uf = UnionField::init(n);
    let mut ans = vec![n-1 ; n];
    for _ in 0..m{
        input!{
            a: usize,
            b: usize,
        }
        uf.merge(a-1, b-1);
        ans[a-1] -= 1;
        ans[b-1] -= 1;
    }
    for _ in 0..k{
        input!{
            c:usize,
            d:usize,
        }
        ans[c-1] -= 1;
        ans[d-1] -= 1;
    }
    //for (a, b) in f{
    //    uf.merge(a-1, b-1);
    //}
    //for (c, d) in b{
    //    ans[c-1] -= 1;
    //    ans[d-1] -= 1;
    //}

    for i in 0..n{
        for j in i+1..n{
            if uf.is_same(i, j){
                ans[i] -=1;
                ans[j] -=1;
            }
        }
    }
    for k in ans{
        print!("{} ", k);
    }
    println!()
}

pub struct UnionField {
    n: usize,
    root: Vec<usize>,
    size: Vec<usize>,
}

impl UnionField{
    pub fn init(s: usize) -> UnionField{
        UnionField{
        n: s,
        root: (0..s).map(|i| i).collect::<Vec<usize>>(),
        size: vec![1; s],
        }
    }

    pub fn find(&mut self, x: usize) -> usize{
        if x == self.root[x] {
            return x;
        }else{
            self.root[x] = self.find(self.root[x]);
            return self.root[x];
        }
    }
    
    pub fn merge(&mut self, x: usize, y: usize){
        let rx = self.find(x);
        let ry = self.find(y);
        if rx==ry{return;}
        let (rb, rs) = if self.size[rx]<self.size[ry] {(rx, ry)} else {(ry, rx)};
        self.root[rb] = rs;
        self.size[rb] += self.size[rs];
        self.size[rs] = 0;
        //self.n -= 1;
    }

    pub fn is_same(&mut self, x: usize, y: usize) -> bool{
        if self.find(x)==self.find(y){return true;}
        else {return false;}
    }
}