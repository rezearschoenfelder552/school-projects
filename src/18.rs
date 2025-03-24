// This is just an example and can be modified based on your specific needs.
use std::vec::Vec;

fn main() {
    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    
    for num in &numbers {
        println!("{}", *num);
    }
}
