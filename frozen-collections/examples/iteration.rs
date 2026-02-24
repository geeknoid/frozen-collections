//! Iterating, mutating values, and converting frozen maps.
//!
//! Frozen maps have immutable keys but mutable values. This example
//! demonstrates iteration, value mutation, and consuming iterators.

#![expect(clippy::print_stdout, clippy::wildcard_imports, reason = "Examples prioritize readability")]

use frozen_collections::*;

fn main() {
    // Create a mutable frozen map
    let mut scores = FzStringMap::new(vec![
        (String::from("Alice"), 85_i32),
        (String::from("Bob"), 92),
        (String::from("Carol"), 78),
        (String::from("Dave"), 95),
    ]);

    println!("Original scores:");
    for (name, &score) in &scores {
        println!("  {name}: {score}");
    }

    // Mutate a single value
    if let Some(score) = scores.get_mut("Carol") {
        *score += 10;
        println!("\nCarol's score after bonus: {score}");
    }

    // Mutate all values
    for score in scores.values_mut() {
        *score = (*score).min(100); // cap at 100
    }

    println!("\nCapped scores:");
    for (name, &score) in &scores {
        println!("  {name}: {score}");
    }

    // Iterate over just keys or just values
    let names: Vec<_> = scores.keys().collect();
    println!("\nAll participants: {names:?}");

    let all_scores: Vec<_> = scores.values().collect();
    println!("All scores: {all_scores:?}");

    println!("Total entries: {}", scores.len());
    println!("Is empty? {}", scores.is_empty());

    // Consuming iterators: into_keys, into_values
    let final_scores: Vec<_> = scores.into_values().collect();
    println!("Final scores (consumed): {final_scores:?}");
}
