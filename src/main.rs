use std::io::*;

fn main() {
    let mut buf = BufReader::new(stdin()).lines();
    let mut read_line = || buf.next().unwrap().unwrap();
}