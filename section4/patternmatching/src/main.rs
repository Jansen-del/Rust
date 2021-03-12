fn how_many(x:i32) -> &'static str{
    match x {
        0 => "no",
        1|2 => "a couple of",
        12 => "a dozen",
        9..=11 => "lots of",
        _ => "a few"

    }
}

pub fn pattern_matching(){
    for x in 0..13{
        println!("{}: I have {} oranges", x, how_many(x));
    }    
}

fn main() {
    pattern_matching()
}