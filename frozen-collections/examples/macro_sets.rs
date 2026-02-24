//! Basic usage of frozen sets created with macros.
//!
//! Demonstrates the short form of the `fz_*_set!` macros and
//! set operations like union, intersection, and difference.

#![expect(clippy::print_stdout, clippy::wildcard_imports, reason = "Examples prioritize readability")]

use frozen_collections::*;

fn main() {
    // A set of known file extensions
    let image_extensions = fz_string_set!({"png", "jpg", "gif", "bmp", "webp"});
    let video_extensions = fz_string_set!({"mp4", "avi", "mkv", "webm", "gif"});

    println!("Is 'png' an image extension? {}", image_extensions.contains("png"));
    println!("Is 'mp4' an image extension? {}", image_extensions.contains("mp4"));

    // Set operations
    let both: Vec<_> = image_extensions.intersection(&video_extensions).collect();
    println!("\nShared between image and video: {both:?}");

    let image_only: Vec<_> = image_extensions.difference(&video_extensions).collect();
    println!("Image only: {image_only:?}");

    let all: Vec<_> = image_extensions.union(&video_extensions).collect();
    println!("All extensions: {all:?}");

    println!("Are image and video disjoint? {}", image_extensions.is_disjoint(&video_extensions));

    // Scalar set with integer values
    let primes = fz_scalar_set!({2_u32, 3_u32, 5_u32, 7_u32, 11_u32, 13_u32});

    println!("\nIs 7 prime? {}", primes.contains(&7));
    println!("Is 9 prime? {}", primes.contains(&9));

    // Iterate over the set
    print!("Known primes:");
    for &p in &primes {
        print!(" {p}");
    }
    println!();
}
