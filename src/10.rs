use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    // Insert some scores into the map
    scores.insert(String::from("Alice"), 10);
    scores.insert(String::from("Bob"), 20);
    scores.insert(String::from("Charlie"), 30);

    // Print out the scores
    println!("Scores: {:?}", scores);
}
