#![allow(dead_code)]
#![allow(non_camel_case_types)]

enum Color{
    Red, 
    Green, 
    Blue, 
    rgbColor
}

fn enums() {
    let c:Color = Color::Red;
    match c {
        Color::Red => println!("R"),
        Color::Green => println!("G"),
        Color::Blue => println!("B"),
        _ => println!("who knows what this is")
    }
}

fn main() {
    println!("Hello, world!");
    enums();
}
