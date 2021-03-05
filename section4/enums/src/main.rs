#![allow(dead_code)]
#![allow(non_camel_case_types)]

enum Color{
    Red, 
    Green, 
    Blue, 
    rgbColor (u8, u8, u8), 
    cmyk {cyan:u8, magenta: u8, yellow: u8, black: u8}
}

fn enums() {
    let c:Color = Color::cmyk{cyan:3, magenta:23, yellow:234, black:0};
    match c {
        Color::Red => println!("R"),
        Color::Green => println!("G"),
        Color::Blue => println!("B"),
        Color::rgbColor (0,0,0) => println!("black"),
        _ => println!("who knows what this is")
        
    }
}

fn main() {
    println!("Hello, world!");
    enums();
}
