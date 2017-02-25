#[allow(dead_code)]
enum Color {
    Red,
    Blue,
    Green,
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

fn main () {
    let color = Color::CMY(122, 17, 40);
    println!("What Color is it?");

    match color {
        Color::Red => println!("The color is red!"),
        Color::Blue => println!("The color is blue!"),
        Color::Green => println!("The color is green!"),
        Color::RGB(r, g, b) => println!("Red: {}, green: {}, blue: {}", r, g, b),
        Color:: HSV(h, s, v) => println!("Hue: {}, saturation: {}, value: {}", h, s, v),
        _ => println!("Whatever!")
    }
}
