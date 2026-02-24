//! Using the Map and Set traits for generic programming.
//!
//! All frozen collections implement the `Map` or `Set` trait, enabling
//! generic functions that work with any frozen collection type — whether
//! created by macros or at runtime.

#![expect(
    clippy::print_stdout,
    clippy::wildcard_imports,
    clippy::trivially_copy_pass_by_ref,
    reason = "Examples prioritize readability"
)]

use frozen_collections::*;

/// Looks up a key in any frozen map and prints the result.
fn lookup_and_print<M>(map: &M, key: &i32)
where
    M: MapQuery<i32, &'static str>,
{
    match map.get(key) {
        Some(value) => println!("  {key} => {value}"),
        None => println!("  {key} => (not found)"),
    }
}

/// Prints a summary of any frozen set.
fn print_set_info<S, T>(label: &str, set: &S)
where
    S: SetIteration<T> + Len,
    for<'a> <S as SetIteration<T>>::Iterator<'a>: Clone,
    T: core::fmt::Debug,
{
    println!("{label} ({} items): {:?}", set.len(), set.iter().collect::<Vec<_>>());
}

fn main() {
    // Macro-created map: specific optimized type, but still implements Map
    let macro_map = fz_scalar_map!({
        1_i32: "one",
        2_i32: "two",
        3_i32: "three",
    });

    // Runtime-created map: different underlying type, same trait
    let runtime_map = FzScalarMap::new(vec![(10, "ten"), (20, "twenty"), (30, "thirty")]);

    // The same generic function works with both macro and runtime maps.
    println!("Macro map lookups:");
    lookup_and_print(&macro_map, &1);
    lookup_and_print(&macro_map, &99);

    println!("\nRuntime map lookups:");
    lookup_and_print(&runtime_map, &20);
    lookup_and_print(&runtime_map, &99);

    // Same generic function works with both macro and runtime sets.
    let set_a = fz_scalar_set!({1_i32, 2_i32, 3_i32, 4_i32, 5_i32});
    let set_b = FzScalarSet::new(vec![10, 20, 30]);

    print_set_info("\nMacro set", &set_a);
    print_set_info("Runtime set", &set_b);
}
