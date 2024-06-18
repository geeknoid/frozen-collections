use frozen_collections::*;

fn main() {
    use_macro();
    use_function();
}

fn use_macro() {
    // Create a frozen map using the frozen_map! macro. This results in
    // the best performance, but it requires that all the keys be known
    // at compile time
    let fm = frozen_map!(
        &str,
        "first_key": (1, "first_value"),
        "second_key": (2, "second_value"),
        "third_key": (3, "third_value"),
        "fourth_key": (4, "fourth_value"),
        "fifth_key": (5, "fifth_value"),
        "sixth_key": (6, "sixth_value"),
        "seventh_key": (7, "seventh_value"),
    );

    assert_eq!(7, fm.len());
    assert!(!fm.is_empty());
    assert!(fm.contains_key("first_key"));
    assert!(!fm.contains_key("eight_key"));
    assert_eq!(Some(&(2, "second_value")), fm.get("second_key"));
    assert_eq!(
        Some((&"third_key".to_string(), &(3, "third_value"))),
        fm.get_key_value("third_key")
    );

    // print out the map's contents
    println!("MAP CONTENTS");
    println!("  {fm:?}");

    // print out all the entries, in random order
    println!("MAP ENTRIES");
    for entry in &fm {
        println!("  {entry:?}");
    }

    // print out all the keys, in random order
    println!("MAP KEYS");
    for key in fm.keys() {
        println!("  {key:?}");
    }

    // print out all the values, in random order
    println!("MAP VALUES");
    for value in fm.values() {
        println!("  {value:?}");
    }
}

fn use_function() {
    // Create a frozen map using the FrozenMap type. This is slightly
    // slower than using the frozen_map! macro, but is necessary when the
    // keys are not known at compile time.

    // The key/value pairs we're loading into the frozen map. Imagine these
    // are determined at runtime.
    let v = vec![
        ("first_key", (1, "first_value")),
        ("second_key", (2, "second_value")),
        ("third_key", (3, "third_value")),
        ("fourth_key", (4, "fourth_value")),
        ("fifth_key", (5, "fifth_value")),
        ("sixth_key", (6, "sixth_value")),
        ("seventh_key", (7, "seventh_value")),
    ];

    let fm = FrozenMap::try_from(v).unwrap();

    assert_eq!(7, fm.len());
    assert!(!fm.is_empty());
    assert!(fm.contains_key(&"first_key"));
    assert!(!fm.contains_key(&"eight_key"));
    assert_eq!(Some(&(2, "second_value")), fm.get(&"second_key"));
    assert_eq!(
        Some((&"third_key", &(3, "third_value"))),
        fm.get_key_value(&"third_key")
    );

    // print out the map's contents
    println!("MAP CONTENTS");
    println!("  {fm:?}");

    // print out all the entries, in random order
    println!("MAP ENTRIES");
    for entry in &fm {
        println!("  {entry:?}");
    }

    // print out all the keys, in random order
    println!("MAP KEYS");
    for key in fm.keys() {
        println!("  {key:?}");
    }

    // print out all the values, in random order
    println!("MAP VALUES");
    for value in fm.values() {
        println!("  {value:?}");
    }
}
