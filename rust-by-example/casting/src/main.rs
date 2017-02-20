// Sppress all warnings from casts which overflow.
#![allow(overflowing_literals)]

fn main() {
    let decimal = 65.4321_f32;

    // Error! No implicit conversion
    // let integer: u8 = decimal;

    let integer = decimal as u8;
    let character = integer as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    // When casting any value to an unsigned type, T,
    // std::T::MAX + 1 is added or subtracted until the value
    // fits into a new value

    // 1000 already fits in a u16
    println!("1000 as u16 is: {}", 1000 as u16);

    // 1000 - 256 - 256 - 256 = 232
    // Under the hood, the first 8 least significant bits(LSB) are kept.
    // while the rest towards the most significant bit(MSB) get truncated.
    println!("1000 as a u8 is: {}", 1000 as u8);

    println!("-1 as a u8 is: {}", (-1i8) as u8);

    println!("128 as a i16 is: {}", 128 as i16);

    println!("128 as a i8 is: {}", 128 as i8);

    println!("1000 as a i8 is: {}", 1000 as i8);
    println!("232 as a i8 is: {}", 232 as i8);
}
