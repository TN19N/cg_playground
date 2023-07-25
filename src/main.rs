use std::{error::Error, io::{BufReader, self, BufRead}};

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf = BufReader::new(io::stdin()).lines();
    let mut read_line = || buf.next().unwrap().unwrap();

    Ok(())
}