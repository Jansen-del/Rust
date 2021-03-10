use std::mem;

fn arrays(){
    let mut  a:[i32; 5] = [1,2,3,4,5];
    println!("array a has {} elements and the first element is {}", a.len(), a[0]);
    a[0] = 321;
    println!("{:?}", a);

    if a != [1,2,3,4,5] {
        println!("arrays are not equal");
    }
    let b = [1;10];
    for i in 0..b.len(){
        println!("b is {}", b[i]);
    }
    println!("b took up {} bytes", mem::size_of_val(&b));
    let mtx:[[f32;3];2] =
    [ 
        [1.0,2.0,3.0],
        [4.0,5.0,6.0]
    ];
    println!("{:?}",mtx);
    for i in 0..mtx.len(){
        for j in 0..mtx[i].len(){
            if i == j{
                println!("matrix at diagonal value is {}",mtx[i][j])
            }
        }
    }
}

fn main(){
    arrays();
}