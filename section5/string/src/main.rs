// there are two types of sttrings in rust
// breezed through introduction to strings in rust
// now going to format macro for strings
// which enters a string variable into a string basically "Hello"+ var

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_must_use)]

use std::thread;
use std::time;

fn main() {
    println!("Hello, world!");
    let name = "jansen";
    let greeting = format!("My name is {}",name);
    println!("{}",greeting);
}
