//! Using the Scalar derive macro with enums for efficient lookup.
//!
//! The `Scalar` derive macro lets you use enum variants as keys in
//! `fz_scalar_map!` and `fz_scalar_set!`, which can produce very
//! fast lookups (dense or sparse array indexing instead of hashing).

#![expect(
    clippy::unwrap_used,
    clippy::print_stdout,
    clippy::wildcard_imports,
    clippy::needless_pass_by_value,
    reason = "Examples prioritize readability"
)]

use frozen_collections::*;

#[derive(Scalar, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Debug)]
enum Color {
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Indigo,
    Violet,
}

#[derive(Scalar, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn main() {
    // Enum keys produce dense array lookups — extremely fast.
    let color_hex = fz_scalar_map!({
        Color::Red: "#FF0000",
        Color::Orange: "#FFA500",
        Color::Yellow: "#FFFF00",
        Color::Green: "#008000",
        Color::Blue: "#0000FF",
        Color::Indigo: "#4B0082",
        Color::Violet: "#EE82EE",
    });

    println!("Red hex: {}", color_hex.get(&Color::Red).unwrap());
    println!("Blue hex: {}", color_hex.get(&Color::Blue).unwrap());

    // Enum set: check membership
    let warm_colors = fz_scalar_set!({Color::Red, Color::Orange, Color::Yellow});

    println!("\nIs Red warm? {}", warm_colors.contains(&Color::Red));
    println!("Is Blue warm? {}", warm_colors.contains(&Color::Blue));

    // Use with generics via the Map trait
    let opposites = fz_scalar_map!({
        Direction::North: Direction::South,
        Direction::South: Direction::North,
        Direction::East: Direction::West,
        Direction::West: Direction::East,
    });

    print_opposites(opposites);
}

fn print_opposites<M>(map: M)
where
    M: Map<Direction, Direction>,
{
    println!("\nDirection opposites:");
    for (dir, opp) in map.iter() {
        println!("  {dir:?} -> {opp:?}");
    }
}
