use frozen_collections::*;

fn main() {
    use_macro();
    use_function();
}

fn use_macro() {
    // Create a frozen set using the frozen_set! macro. This results in
    // the best performance, but it requires that all the values be known
    // at compile time
    let fs = frozen_set!(
        &str,
        "first_value",
        "second_value",
        "third_value",
        "fourth_value",
        "fifth_value",
        "sixth_value",
        "seventh_value",
    );

    assert_eq!(7, fs.len());
    assert!(!fs.is_empty());
    assert!(fs.contains("first_value"));
    assert!(!fs.contains("eight_value"));

    // print out the set's contents
    println!("SET CONTENTS");
    println!("  {fs:?}");

    // print out all the values, in random order
    println!("SET VALUES");
    for value in &fs {
        println!("  {value:?}");
    }
}

fn use_function() {
    // Create a frozen set using the FrozenSet type. This is slightly
    // slower than using the frozen_set! macro, but is necessary when the
    // values are not known at compile time.

    // The values we're loading into the frozen set. Imagine these
    // are determined at runtime.
    let v = vec![
        "first_value",
        "second_value",
        "third_value",
        "fourth_value",
        "fifth_value",
        "sixth_value",
        "seventh_value",
    ];

    let fs = FrozenSet::try_from(v).unwrap();

    assert_eq!(7, fs.len());
    assert!(!fs.is_empty());
    assert!(fs.contains(&"first_value"));
    assert!(!fs.contains(&"eight_value"));

    // print out the set's contents
    println!("SET CONTENTS");
    println!("  {fs:?}");

    // print out all the values, in random order
    println!("SET VALUES");
    for value in &fs {
        println!("  {value:?}");
    }
}
