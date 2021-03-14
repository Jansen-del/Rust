// Hashmap
// allows assigning key value pair (json or python dict?)

use std::collections::HashMap;

fn main(){
    let mut shapes = HashMap::new();
    shapes.insert(String::from("triangle"), 3);
    shapes.insert(String::from("square"),4);
    println!("A square has {} sides", shapes["square".into()]);

    for (key, value) in &shapes{
        println!("{}: {}", key, value);
    }
    shapes.insert("square".into(), 5);
    // inserting into existing keys will simply change ... 
    //the value not add a new one
    println!("{:?}",shapes);

    shapes.entry("circle".into()).or_insert(1);
    println!("{:?}", shapes )
}