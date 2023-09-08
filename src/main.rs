use std::io::{self, BufRead};

fn main() {
    let mut buffer = io::stdin().lock().lines();
    let mut read_line = || buffer.next().unwrap().unwrap();
}
