// tuples are similar to arrays but 
//they can have mutilple data types


fn sum_product(x:i32, y:i32) -> (i32,i32) {
    (x+y,x*y)
}
fn tuples(){
    let x = 2;
    let y = 8;
    let sp = sum_product(x, y);
    println!("{:?}", sp);
    println!("{0} + {1} = {2}, {0} * {1} = {3}", x, y, sp.0, sp.1);
    //destructuring: taking existing tsructur eand split to different vars
    let (a,b) = sp;
    println!("{0} + {1} = {2}, {0} * {1} = {3}", x, y, a,b);

    // we can have tuples of tuples
    let sp2 = sum_product(3, 4);
    let combined = (sp, sp2);
    println!("{:?}",combined)

}

fn main(){
    tuples();
}