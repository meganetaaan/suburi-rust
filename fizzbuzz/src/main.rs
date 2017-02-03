fn main() {
    use std::io;
    let mut input = String::new(); // guess is mutable

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");

    let input: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };

    println!("you entered {}", input);

    let mut arr: Vec<String> = Vec::new();
    for x in 1..input {
        arr.push(fizzbuzz(x));
    }

    println!("{}", arr.iter().cloned().collect::<String>());
}

fn fizzbuzz (x: i32) -> String {
    match (x % 3, x % 5) {
        (0, 0) => "fizzbuzz".to_string(),
        (_, 0) => "buzz".to_string(),
        (0, _) => "fizz".to_string(),
        (_, _) => x.to_string()
    }
}
