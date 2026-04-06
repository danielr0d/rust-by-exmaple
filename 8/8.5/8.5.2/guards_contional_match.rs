fn main() {
    let number: u8 = 5;

    match number {
        i if i == 0 => println!("zero"),
        i if i > 0 => println!("greater than zero"),
        _ => unreachable!("should never happen"),
    }
}
