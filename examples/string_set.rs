use frozen_collections::*;

// Declare a static string set. This results in the set being embedded directly into the
// binary image.
fz_string_set!(static MY_STRING_SET: MyStringSetType<&str> = {
    "Alice",
    "Bob",
});

// Demonstrates how to use a static frozen string set.
fn use_static() {
    assert!(MY_STRING_SET.contains("Alice"));
    assert!(MY_STRING_SET.contains("Bob"));
    assert!(!MY_STRING_SET.contains("Fred"));
}

// Demonstrates how to create a frozen set as a local variable.
fn use_local() {
    let ss = fz_string_set!("Alice", "Bob");

    assert!(ss.contains("Alice"));
    assert!(ss.contains("Bob"));
    assert!(!ss.contains("Fred"));
}

// Demonstrates how to use a facade to create a frozen string set.
//
// Using a facade is slower than using the fz_string_set! macro, but it
// allows values that are determined at runtime, whereas the macro requires
// the values to be known at compile time.
fn use_facade() {
    let ss = FzStringSet::new(vec!["Alice".to_string(), "Bob".to_string()]);

    assert!(ss.contains("Alice"));
    assert!(ss.contains("Bob"));
    assert!(!ss.contains("Fred"));
}

fn main() {
    use_static();
    use_local();
    use_facade();
}
