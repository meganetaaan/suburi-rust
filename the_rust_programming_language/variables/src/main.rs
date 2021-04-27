const MAX_POINTS: u32 = 100_000;
fn main() {
    let x: u32 = 5;
    println!("The value of x is: {}", x);
    let x = "redefined";
    println!("The value of x is: {}", x);

    let guess: u32 = "42".parse().expect("Not a number!");

    let number: i32 = -1; // basic
    let digit: u32 = 98_222;
    let hex: u8 = 0xff;
    let oct: u8 = 0o77;
    let binary: u8 = 0b1111_0000;
    let byte: u8 = b'A';

    let float = 2.0;
    let double: f64 = 3.0; // basic

    let sum: u16 = hex as u16 + oct as u16;
    let diff = digit - oct as u32;
    let product = binary * 1;
    let quotient = digit as f64 / 2.0;
    let remainder = digit % 5;

    let t = true;
    let f = false;

    let c = 'z';
    let z = 'Ｚ';
    let heart_eyed_cat = '�';
}
