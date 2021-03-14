/*
    learning to use vectors
    vectors are used when you don't have any array of fixed size
    arrays are fixed size but vectors are unlimited 
*/

fn vectors(){
    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);
    println!("a = {:?}",a);
    a.push(44);
    println!("a = {:?}",a);

    let idx:usize = 0; // when getting index, you need to assign the size of the system
    a[idx] = 352;
    println!("a[0] = {}",a[idx]);
    
    // get function returns an option type
    match a.get(0){
        //the some keywords checks if a result is being returned
        Some(_x) => println!("Vector element is {}", a[0]),
        //None value is run when nothing is returned
        None => println!("Empty")

    }
    for x in &a {
        println!("{}", x);
    }

    // using keyword pop removes the last element in vector

    let x = a.pop();
    println!("x = {:?}, remaining array is {:?}", x, a);

    while let Some(x) = a.pop(){
        println!("x is {}",x)
    }

}

fn main(){
    vectors()
}