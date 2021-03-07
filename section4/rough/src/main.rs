fn main() {
    let x =  3.0;
    let y = 1.0;

    //option  
    let result = 
        if y!= 0.0 {Some(x/y)} else {None};
    // println!{"result is {}", result}
    match result {
        Some(z) => println!("{}/{}={}",x, y, z),
        None => println!("cannot divide by zero")
    }
}
