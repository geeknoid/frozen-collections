use frozen_collections::*;

fz_sequence_set!(pub static MY_SET1: MySetType1<i32> = { 0, 2 });
fz_sequence_set!(pub static MY_SET2: MySetType2<i32> = { 0, 1, 2, 3, 4, 5, 6, 7, 8, 9 });
fz_sequence_set!(pub static MY_SET3: MySetType3<i32> = { 0, 10, 200, 3000, 40000 });
fz_sequence_set!(pub static MY_SET4: MySetType4<i32> = { 0, 10, 200, 3000, 40000, 500_000, 6_000_000, 70_000_000, 800_000_000 });
fz_sequence_set!(pub static MY_SET5: MySetType5<i32> = { 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 200, 3000, 40000, 500_000, 6_000_000, 70_000_000, 800_000_000 });

fn main() {
    use_macro();
    use_function();
    use_enum();
}

#[derive(Sequence, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Color {
    Red,
    Green,
    Blue,
}

fn use_enum() {
    let fs = FzSequenceSet::new(vec![Color::Red, Color::Green, Color::Blue]);
    _ = fs.get(&Color::Red);
}

fn use_macro() {
    // Create a frozen set using the frozen_set! macro. This results in
    // the best performance, but it requires that all the values be known
    // at compile time
    let fs = fz_string_set!(
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

    let fs = FzHashSet::new(v);

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
