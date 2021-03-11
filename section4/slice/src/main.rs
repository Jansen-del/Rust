// A slice is used to take a section of an array 


// The fucntion below says we are expecting a section of array of type i32
fn use_slice(slice: &[i32]){
    println!("the first element of slice is {} and length is {}", slice[0], slice.len());
    println!("array", slice);
}

fn main() {
    let b = [1,2,3,4,5];
    use_slice(&b[0..4]);
    // make sure to include the & sign so it acts as a pointer
}
