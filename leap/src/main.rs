fn main() {
    use std::io;
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let split : Vec<&str> = input.split(' ').collect();
    let a : i64 = split[0].trim().parse::<i64>().unwrap();
    let b : i64 = split[1].trim().parse::<i64>().unwrap();
    let ans = count_leap(b) - count_leap(a - 1);
    println!("{}", ans);
}

fn count_leap(year: i64) -> i64 {
    (year / 4) + (year / 100) + (year / 400)
}
