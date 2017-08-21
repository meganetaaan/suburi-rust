use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::collections::VecDeque;

fn tail<R: Read>(reader: BufReader<R>) {
    let mut que: VecDeque<String> = VecDeque::new();
    for xs in reader.lines() {
        if que.len() == 10 {
            que.pop_front();
        }
        que.push_back(xs.unwrap());
    }
    for xs in que.iter() {
        println!("{}", xs);
    }
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() == 1 {
        tail(BufReader::new(io::stdin()));
    } else {
        match File::open(&args[1]) {
            Ok(file) => tail(BufReader::new(file)),
            Err(_) => println!("{} not found", &args[1]),
        }
    }
}
