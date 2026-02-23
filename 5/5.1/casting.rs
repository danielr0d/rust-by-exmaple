#![allow(overflowing_literals)]

fn main() {
    let decimal = 65.4321_f32;

    let integer: u8 = decimal;

    let integer = decimal as u8;
    let character = integer as char;

    let character = decimal as char;

    print!("Casting: {} -> {} -> {}", decimal, integer, character);

    print!("1000 as a u16 is: {}", 1000 as u16);

}


