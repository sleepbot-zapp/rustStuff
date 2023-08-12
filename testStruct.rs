// An attribute to hide warnings for unused code.
#![allow(dead_code)]
use std::fmt;

struct Point {
    x: u8,
    y: u8,
}

struct Rectangle{
    start: Point,
    end: Point,
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl fmt::Debug for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Rectangle Diagonal @ {:?} to {:?}", self.start, self.end)
    }
}

fn main(){
    let a = Rectangle{start:Point{x:1, y:0}, end:Point{x:0, y:1}};
    println!("{:?}", a);
}
