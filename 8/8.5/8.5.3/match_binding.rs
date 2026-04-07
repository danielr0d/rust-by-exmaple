fn age() -> u32  {
    15
}

fn main() {
    println!("tell me what type of person you are");

    match age() {
        0 => println!("i havent celebrated my first birthday yet"),

        n @ 1 ..= 13 => println!("im a child of age {:?}", n),
        n @ 13 ..= 19 => println!("im a teen of age {:?}", n),
        n @ (1 | 7 | 15 | 13) => println!("im a teen of age {:?}", n),
        n => println!("im a old person of age {:?}",  n)
    }
}
