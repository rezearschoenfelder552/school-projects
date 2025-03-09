// Generate a random Rust code snippet
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let x: u8 = rng.gen_range(0..255);
    println!("Random number between 0 and 255: {}", x);
}
