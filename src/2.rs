use std::fs::File;
use std::io::Write;

fn main() {
    let mut file = File::create("hello.txt").unwrap();
    write!(file, "Hello, world!").unwrap();
}
