use frozen_collections::*;

// Declare a static string map. This results in the map being embedded directly into the
// binary image.
fz_string_map!(static MY_STRING_MAP: MyStringMapType<&str, i32> = {
    "Alice": 1,
    "Bob": 2,
});

// Demonstrates how to use a static frozen string map.
fn use_static() {
    assert_eq!(Some(&1), MY_STRING_MAP.get("Alice"));
    assert_eq!(Some(&2), MY_STRING_MAP.get("Bob"));
    assert_eq!(None, MY_STRING_MAP.get("Fred"));
}

// Demonstrates how to create a frozen map as a local variable.
fn use_local() {
    let sm = fz_string_map!(
        "Alice": 1,
        "Bob": 2
    );

    assert_eq!(Some(&1), sm.get("Alice"));
    assert_eq!(Some(&2), sm.get("Bob"));
    assert_eq!(None, sm.get("Fred"));
}

struct MyStruct {
    map: MyStringMapType,
}

// Demonstrates how to embed a frozen hash map into a struct. This makes it possible to have
// multiple instances of a frozen map with common keys, but unique values.
fn use_struct() {
    let mut ms = MyStruct {
        map: MY_STRING_MAP.clone(),
    };

    // set a custom value
    if let Some(v) = ms.map.get_mut("Alice") {
        *v = 3;
    }

    assert_eq!(Some(&3), ms.map.get("Alice"));
    assert_eq!(Some(&2), ms.map.get("Bob"));
    assert_eq!(None, ms.map.get("Fred"));
}

// Demonstrates how to use a facade to create a frozen string map.
//
// Using a facade is slower than using the fz_string_map! macro, but it
// allows keys that are determined at runtime, whereas the macro requires
// the keys to be known at compile time.
fn use_facade() {
    let sm = FzStringMap::new(vec![("Alice".to_string(), 1), ("Bob".to_string(), 2)]);

    assert_eq!(Some(&1), sm.get("Alice"));
    assert_eq!(Some(&2), sm.get("Bob"));
    assert_eq!(None, sm.get("Fred"));
}

fn main() {
    use_static();
    use_struct();
    use_local();
    use_facade();
}
