//! Creating frozen collections from runtime data.
//!
//! When the keys aren't known at compile time, use the `Fz*` types
//! directly. They analyze data at construction time and pick the
//! optimal strategy dynamically.

#![expect(
    clippy::unwrap_used,
    clippy::print_stdout,
    clippy::wildcard_imports,
    reason = "Examples prioritize readability"
)]

use frozen_collections::*;

fn main() {
    // Build a map from data that might come from a file or database.
    let entries = vec![("apple", 3), ("banana", 7), ("cherry", 2), ("date", 5), ("elderberry", 1)];

    let inventory = FzStringMap::new(entries);

    println!("Inventory ({} items):", inventory.len());
    for (fruit, &count) in &inventory {
        println!("  {fruit}: {count}");
    }

    println!("\nBananas in stock: {}", inventory.get("banana").unwrap());
    println!("Have figs? {}", inventory.contains_key("fig"));

    // FzHashMap: works with any hashable key type
    let point_labels: FzHashMap<(i32, i32), &str> = FzHashMap::new(vec![((0, 0), "origin"), ((1, 0), "unit-x"), ((0, 1), "unit-y")]);

    println!("\nPoint (0,0): {}", point_labels.get(&(0, 0)).unwrap());

    // FzScalarMap: optimized for integer keys
    let error_messages: FzScalarMap<i32, &str> = FzScalarMap::new(vec![
        (1, "File not found"),
        (2, "Permission denied"),
        (3, "Connection refused"),
        (4, "Timeout"),
    ]);

    println!("\nError 2: {}", error_messages.get(&2).unwrap());

    // FzOrderedMap: for keys that implement Ord; uses binary/Eytzinger search
    let thresholds: FzOrderedMap<i32, &str> = FzOrderedMap::new(vec![(0, "zero"), (10, "low"), (50, "medium"), (90, "high")]);

    println!("\nThreshold 50: {}", thresholds.get(&50).unwrap());

    // Sets from runtime data
    let allowed_origins = FzStringSet::new(vec![String::from("https://example.com"), String::from("https://api.example.com")]);

    let origin = "https://example.com";
    println!("\nIs '{origin}' allowed? {}", allowed_origins.contains(origin));
}
