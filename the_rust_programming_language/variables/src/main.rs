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

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    let arr = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
            "August", "September", "October", "November", "December"];

    let first = arr[0];
    let second = arr[1];

    let idx = 10;
    let tenth = arr[idx];
    println!("The value of element is: {}", tenth);
}
