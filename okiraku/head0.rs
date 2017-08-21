use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn head<R: Read>(reader: BufReader<R>) {
    for xs in reader.lines().take(10) {
        println!("{}", xs.unwrap());
    }
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() == 1 {
        head(BufReader::new(io::stdin()));
    } else {
        match File::open(&args[1]) {
            Ok(file) => head(BufReader::new(file)),
            Err(_) => println!("{} not found", &args[1]),
        }
    }
}
