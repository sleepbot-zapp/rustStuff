// An attribute to hide warnings for unused code.
#![allow(dead_code)]

enum BIT{
    ZERO,
    ONE
}

type Bit = BIT;

fn main(){
    let status = BIT::ONE;
    match status{
        BIT::ZERO => println!("OFF"),
        Bit::ONE => println!("ON")
    }
}
