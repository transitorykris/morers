use std::env;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file: File;

    if args.len() < 2 {
        // This case is really for handling stdin
        loop {
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(0) => {
                    return // EOF we're done
                }
                Ok(_) => {
                    println!("{}", input);
                }
                Err(_) => {
                    panic!("something went wrong!");
                }
            }
        }
    } else {
        // attempt to read from the path given
        // XXX handling a raw string here is probably not what we want to do, use path?
        file = File::open(&args[1]).unwrap();   // TODO check for errors

    }

    let mut buf_reader = BufReader::new(file);

    loop {
        let mut contents = String::new();   // may be faster to clear it than allocate new?
        match buf_reader.read_line(&mut contents) {
            Ok(0) => {
                return // we're done
            }
            Ok(_) => {
                println!("{}", contents);   // TODO: a better print, there will be two new lines
            }
            Err(_) => {
                panic!("something went wrong");
            }
        }
    }
}
