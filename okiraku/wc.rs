use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn wc<R: Read>(reader: BufReader<R>) {
    let mut word = 0;
    let mut line = 0;
    let mut size = 0;
    let mut inword = false;
    for c in reader.bytes() {
        let code = c.unwrap() as char;
        if code.is_whitespace() {
            inword = false;
            if code == '\n' {
                line += 1;
            }
        } else if !inword {
            inword = true;
            word += 1;
        }
        size += 1;
    }
    println!("lines: {}, words: {}, chars: {}", line, word, size);
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 2 {
        wc(BufReader::new(io::stdin()));
    } else {
        match File::open(&args[1]) {
            Ok(file) => wc(BufReader::new(file)),
            Err(_) => println!("{} not found", &args[1]),
        }
    }
}
