use std::io;

fn pascal(z : usize) -> Vec<Vec<u32>> {
    let mut x : Vec<Vec<u32>> = Vec::new();
    for i in 0..z{
        x.push(vec![1]);
        for j in 0.. i{
            if i >= 2 && j != 0 {
                let n = x[i - 1][j - 1] + x[i - 1][j];
                x[i].push(n);
            }
            if j==i-1 {
                x[i].push(1)
            }
        }
    }
    x
}

fn print_pascal(x: Vec<Vec<u32>>){
    for i in x.iter(){
        for j in i.iter(){
            print!("{} ", j)
        }
        print!("\n")
    }
}


fn main() {
    let mut y : String = String::new();
    io::stdin()
        .read_line(&mut y)
        .expect("Fail");
    let z: usize = y.trim().parse().unwrap();
    let x = pascal(z);
    print_pascal(x);
}