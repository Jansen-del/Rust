fn how_many(x:i32) -> &`static str{
    match x {
        0 => "no"
    }
}

pub fn pattern_matching(){
    for x in 1..13{
        println!("{}: I have {} orange(s)", x, how_many(x));
    }    
}

fn main() {
    pattern_matching()
}