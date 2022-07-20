use std::env;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    let file_name = env::args().nth(1);

    let mut reader: Box<dyn BufRead> = match file_name {
        None => {
            // use stdin
            Box::new(BufReader::new(io::stdin()))
        }
        Some(file_name) => {
            // use this file
            Box::new(BufReader::new(File::open(file_name).unwrap())) // XXX need to check for errors!
        }
    };

    let mut count = 0;
    loop {
        let mut contents = String::new();   // may be faster to clear it than allocate new?
        match reader.read_line(&mut contents) {
            Ok(0) => {
                return // we're done
            }
            Ok(_) => {
                print!("{}", contents);
                count = count + 1;
                if count == 40 {
                    print!("--more--");
                    break;
                }
            }
            Err(_) => {
                panic!("something went wrong");
            }
        }
    }
}
