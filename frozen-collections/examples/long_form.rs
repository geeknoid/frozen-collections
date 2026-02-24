//! Using the long-form macro syntax to create static and typed collections.
//!
//! The long form lets you declare a type alias for the collection,
//! which is essential for static declarations and for naming the type
//! in struct fields or function signatures.

#![expect(
    clippy::unwrap_used,
    clippy::print_stdout,
    clippy::wildcard_imports,
    reason = "Examples prioritize readability"
)]

use frozen_collections::*;

// Static frozen map: lives for the entire program, no heap allocation,
// and the type alias `ConfigMap` can be used in signatures.
fz_string_map!(static CONFIG: ConfigMap<&'static str, i32>, {
    "max_retries": 3,
    "timeout_ms": 5000,
    "batch_size": 100,
    "port": 8080,
});

// Static frozen set.
fz_string_set!(static RESERVED_WORDS: ReservedWords<&'static str>, {
    "fn", "let", "mut", "pub", "use", "mod", "impl", "trait", "struct", "enum",
});

fn show_config(config: &ConfigMap) {
    println!("Configuration ({} entries):", config.len());
    for (key, value) in config {
        println!("  {key} = {value}");
    }
}

fn is_reserved(word: &str) -> bool {
    RESERVED_WORDS.contains(word)
}

fn main() {
    show_config(&CONFIG);

    println!("\nTimeout: {}ms", CONFIG.get("timeout_ms").unwrap());
    println!("Port: {}", CONFIG.get("port").unwrap());

    println!("\nIs 'fn' a reserved word? {}", is_reserved("fn"));
    println!("Is 'foo' a reserved word? {}", is_reserved("foo"));

    // Long form with `let` to get a named local type.
    fz_scalar_map!(let codes: StatusCodes<u16, &'static str>, {
        200_u16: "OK",
        404_u16: "Not Found",
        500_u16: "Error",
    });

    println!("\nStatus codes:");
    for (&code, &desc) in &codes {
        println!("  {code}: {desc}");
    }
}
