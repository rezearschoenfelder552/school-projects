// This is an example Rust code for generating random numbers.
use rand::prelude::*;

fn main() {
    let mut rng = thread_rng();
    
    // Seed the random number generator with a specific seed value.
    rng.seed_from_u64(42);
    
    // Generate 10 random integers between 0 and 99.
    for _ in 0..10 {
        println!("{}", rng.gen_range(0, 99));
    }
}
