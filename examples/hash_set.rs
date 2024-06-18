use frozen_collections::*;

// The value type we use in our example sets
#[derive(Eq, PartialEq, Hash)]
struct Key {
    pub name: &'static str,
    pub age: i32,
}

// Declare a static hash set. This results in the set being embedded directly into the
// binary image.
fz_hash_set!(static MY_HASH_SET: MyHashSetType<&Key> = {
    &Key { name: "Alice", age: 30},
    &Key { name: "Bob", age: 40},
});

// Demonstrates how to use a static frozen hash map.
fn use_static() {
    assert!(MY_HASH_SET.contains(&Key {
        name: "Alice",
        age: 30
    }));
    assert!(MY_HASH_SET.contains(&Key {
        name: "Bob",
        age: 40
    }));

    assert!(!MY_HASH_SET.contains(&Key {
        name: "Alice",
        age: 31
    }));
    assert!(!MY_HASH_SET.contains(&Key {
        name: "Bob",
        age: 41
    }));
}

// Demonstrates how to create a frozen map as a local variable.
fn use_local() {
    let hs = fz_hash_set!(
        Key {
            name: "Alice",
            age: 30
        },
        Key {
            name: "Bob",
            age: 40
        }
    );

    assert!(hs.contains(&Key {
        name: "Alice",
        age: 30
    }));
    assert!(hs.contains(&Key {
        name: "Bob",
        age: 40
    }));

    assert!(!hs.contains(&Key {
        name: "Alice",
        age: 31
    }));
    assert!(!hs.contains(&Key {
        name: "Bob",
        age: 41
    }));
}

// Demonstrates how to use a facade to create a frozen hash set.
//
// Using a facade is slower than using the fz_hash_set! macro, but it
// allows values that are determined at runtime, whereas the macro requires
// the values to be known at compile time.
fn use_facade() {
    let hs = FzHashSet::new(vec![
        (Key {
            name: "Alice",
            age: 30,
        }),
        (Key {
            name: "Bob",
            age: 40,
        }),
    ]);

    assert!(hs.contains(&Key {
        name: "Alice",
        age: 30
    }));
    assert!(hs.contains(&Key {
        name: "Bob",
        age: 40
    }));

    assert!(!hs.contains(&Key {
        name: "Alice",
        age: 31
    }));
    assert!(!hs.contains(&Key {
        name: "Bob",
        age: 41
    }));
}

fn main() {
    use_static();
    use_local();
    use_facade();
}
