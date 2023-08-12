#![allow(dead_code)] // An attribute to hide warnings for unused code.
#![allow(unused_assignments)] // An attribute to hide warnings for declared but not read variables.
use std::fmt;

struct Number {
    value: i8,
}

impl fmt::Debug for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Number({})", self.value)
    }
}


impl From<i8> for Number {
    fn from(item: i8) -> Self {
        Number{value: item}
    }fr
}


fn main(){
    let num = Number::from(30);
    println!("Number is {:?}", num);
}
