use frozen_collections::*;

// The value type we use into our example sets
#[derive(Eq, PartialEq, Ord, PartialOrd)]
struct Key {
    pub name: &'static str,
    pub age: i32,
}

// Declare a static ordered set. This results in the set being embedded directly into the
// binary image.
fz_ordered_set!(static MY_ORDERED_SET: MyOrderedSetType<&Key> = {
    &Key { name: "Alice", age: 30},
    &Key { name: "Bob", age: 40},
});

// Demonstrates how to use a static frozen ordered set.
fn use_static() {
    assert!(MY_ORDERED_SET.contains(&Key {
        name: "Alice",
        age: 30
    }));
    assert!(MY_ORDERED_SET.contains(&Key {
        name: "Bob",
        age: 40
    }));

    assert!(!MY_ORDERED_SET.contains(&Key {
        name: "Alice",
        age: 31
    }));
    assert!(!MY_ORDERED_SET.contains(&Key {
        name: "Bob",
        age: 41
    }));
}

// Demonstrates how to create a frozen set as a local variable.
fn use_local() {
    let os = fz_ordered_set!(
        Key {
            name: "Alice",
            age: 30
        },
        Key {
            name: "Bob",
            age: 40
        }
    );

    assert!(os.contains(&Key {
        name: "Alice",
        age: 30
    }));
    assert!(os.contains(&Key {
        name: "Bob",
        age: 40
    }));

    assert!(!os.contains(&Key {
        name: "Alice",
        age: 31
    }));
    assert!(!os.contains(&Key {
        name: "Bob",
        age: 41
    }));
}

// Demonstrates how to use a facade to create a frozen ordered set.
//
// Using a facade is slower than using the fz_ordered_set! macro, but it
// allows keys that are determined at runtime, whereas the macro requires
// the keys to be known at compile time.
fn use_facade() {
    let os = FzOrderedSet::new(vec![
        (Key {
            name: "Alice",
            age: 30,
        }),
        (Key {
            name: "Bob",
            age: 40,
        }),
    ]);

    assert!(os.contains(&Key {
        name: "Alice",
        age: 30
    }));
    assert!(os.contains(&Key {
        name: "Bob",
        age: 40
    }));

    assert!(!os.contains(&Key {
        name: "Alice",
        age: 31
    }));
    assert!(!os.contains(&Key {
        name: "Bob",
        age: 41
    }));
}

fn main() {
    use_static();
    use_local();
    use_facade();
}
