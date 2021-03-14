// learning to use generics which allow me to assign any data type to a struct?
// whole point of generics is to allow us to use any type of data 

struct Point<T> // this section is telling the code that i am sending a random data type
{
    x:T,
    y:T
}

struct Line<T>{
    start:Point<T>,
    end:Point<T>
}

fn generics(){
    let a = Point{x: 0, y:3};
    let b = Point{x: 1.3 , y:5.7};
    // here I'm defining A with ints and b with floats
    // println!("{}", a)
}
fn main()
{
    generics();
}