fn main() {
    println!("Hello, world!");
    let x = plus_one(5);

    // below is a statement
    let mut y = 6;

    // a statement cannot be a formula
    // let x = (let y = 6); // compile error

    // a block is a formula
    y = {
        let x = 3;
        x + 1
    };

    another_function(x, y);
}

fn five() -> i32 {
    // This just works
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {} and y is: {}", x, y);
}