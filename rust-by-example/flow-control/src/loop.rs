fn main() {
    let mut count = 0u32;
    loop {
        count += 1;

        if count == 3 {
            println!("three");
            continue;
        }

        println!("{}", count);

        if count == 10 {
            println!("OK that's enough");
            break;
        }
    }
}

