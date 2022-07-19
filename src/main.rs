use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    let file = File::open("/users/krisfoster/Desktop/jane.txt").unwrap();   // TODO check for errors
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
