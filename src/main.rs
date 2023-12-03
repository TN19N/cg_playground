fn main() {
    let read_line = || std::io::stdin().lines().next().unwrap().unwrap();

    println!("{}", read_line())
}
