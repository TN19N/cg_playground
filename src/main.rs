use std::{
    io::{
        self,
        BufRead,
        BufReader
    },
    error::Error,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf = BufReader::new(io::stdin()).lines();
    let mut read_line = || buf.next().unwrap().unwrap().trim().to_string();

    Ok(())
}