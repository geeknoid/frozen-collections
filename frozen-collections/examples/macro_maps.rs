//! Basic usage of frozen maps created with macros.
//!
//! Demonstrates the short form of the `fz_*_map!` macros to create
//! compile-time-optimized maps for different key types.

#![expect(
    clippy::unwrap_used,
    clippy::print_stdout,
    clippy::wildcard_imports,
    reason = "Examples prioritize readability"
)]

use frozen_collections::*;

fn main() {
    // String-keyed map: the macro picks the best strategy for string keys
    // (e.g., length-based hashing, substring hashing, or classic hashing).
    let capitals = fz_string_map!({
        "France": "Paris",
        "Japan": "Tokyo",
        "Brazil": "Brasilia",
        "Australia": "Canberra",
        "Egypt": "Cairo",
    });

    println!("Capital of Japan: {}", capitals.get("Japan").unwrap());
    println!("Contains France? {}", capitals.contains_key("France"));
    println!("Contains India? {}", capitals.contains_key("India"));

    // Integer-keyed map: uses scalar optimizations (dense/sparse lookup, etc.)
    let http_status = fz_scalar_map!({
        200_u16: "OK",
        201_u16: "Created",
        204_u16: "No Content",
        400_u16: "Bad Request",
        404_u16: "Not Found",
        500_u16: "Internal Server Error",
    });

    println!("\nHTTP 200: {}", http_status.get(&200).unwrap());
    println!("HTTP 404: {}", http_status.get(&404).unwrap());
    println!("HTTP 418 exists? {}", http_status.contains_key(&418));

    // Hash map with composite keys
    let grid = fz_hash_map!({
        (0_i32, 0_i32): "origin",
        (1_i32, 0_i32): "east",
        (0_i32, 1_i32): "north",
    });

    println!("\nPoint (0,0): {}", grid.get(&(0, 0)).unwrap());

    // Iterating over any frozen map
    println!("\nAll capitals:");
    for (country, capital) in &capitals {
        println!("  {country} -> {capital}");
    }
}
