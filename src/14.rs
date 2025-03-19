fn main() {
    let mut numbers = vec![1, 2, 3];
    let num_to_add = 5;
    
    for i in 0..num_to_add {
        numbers.push(i);
    }
    
    println!("Numbers: {:?}", numbers);
}
