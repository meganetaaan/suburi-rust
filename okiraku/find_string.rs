use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn find_string<R: Read>(reader: BufReader<R>, key: &String) {
    let mut n = 1;
    for xs in reader.lines() {
        let s = xs.unwrap();
        match s.find(key) {
            Some(_) => println!("{}: {}", n, s),
            None => (),
        }
        n += 1;
    }
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 2 {
        println!("usage: find0 keyword [file_name]");
    } else if args.len() < 3 {
        find_string(BufReader::new(io::stdin()), &args[1]);
    } else {
        match File::open(&args[2]) {
            Ok(file) => find_string(BufReader::new(file), &args[1]),
            Err(_) => println!("{} not found", &args[1]),
        }
    }
}
