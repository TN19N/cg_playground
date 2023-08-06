use std::io::*;
fn main() {
    let mut b=BufReader::new(stdin()).lines();
    let mut r=||b.next().unwrap().unwrap();
}