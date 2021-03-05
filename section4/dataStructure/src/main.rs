#![allow(non_snake_case)]
#![allow(dead_code)]

//creating a struct/dictionary structure to store GPS coordinates by default
struct Point {
    x:f64,
    y:f64
}

struct  Line{
    start:Point,
    end:Point
}

fn structures(){
    let p1 = Point {x:3.0, y:4.0};
    println!("point p is a ({}, {}) coordintaes", p1.x, p1.y);
    let p2 = Point {x:15.0, y:-79.0};
    println!("Point p2 is ({}, {}) coordinates ", p2.x, p2.y)
    let myLine = Line {start:p1, end:p2}
}



fn main() {
    println!("Hello, world!");
    structures()
}
