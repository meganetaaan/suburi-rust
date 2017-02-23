fn main() {
    let pair = (0, 2);

    println!("Tell me about {:?}", pair);

    match pair {
        (0, y) => println!("First is `0` and second is {:?}", y),
        (x, 0) => println!("First is {} and second is `0`", x),
        _ => println!("it doesn't match"),
    }
}
