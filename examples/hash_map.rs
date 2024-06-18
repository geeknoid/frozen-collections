use frozen_collections::*;

// The key type we use to index into our example maps
#[derive(Eq, PartialEq, Hash)]
struct Key {
    pub name: &'static str,
    pub age: i32,
}

// Declare a static hash map. This results in the map being embedded directly into the
// binary image.
fz_hash_map!(static MY_HASH_MAP: MyHashMapType<&Key, i32> = {
    &Key { name: "Alice", age: 30}: 1,
    &Key { name: "Bob", age: 40}: 2,
});

// Demonstrates how to use a static frozen hash map.
fn use_static() {
    assert_eq!(
        Some(&1),
        MY_HASH_MAP.get(&Key {
            name: "Alice",
            age: 30
        })
    );
    assert_eq!(
        Some(&2),
        MY_HASH_MAP.get(&Key {
            name: "Bob",
            age: 40
        })
    );

    assert_eq!(
        None,
        MY_HASH_MAP.get(&Key {
            name: "Alice",
            age: 31
        })
    );
    assert_eq!(
        None,
        MY_HASH_MAP.get(&Key {
            name: "Bob",
            age: 41
        })
    );
}

// Demonstrates how to create a frozen map as a local variable.
fn use_local() {
    let hm = fz_hash_map!(
        Key { name: "Alice", age: 30}: 1,
        Key { name: "Bob", age: 40}: 2
    );

    assert_eq!(
        Some(&1),
        hm.get(&Key {
            name: "Alice",
            age: 30
        })
    );
    assert_eq!(
        Some(&2),
        hm.get(&Key {
            name: "Bob",
            age: 40
        })
    );

    assert_eq!(
        None,
        hm.get(&Key {
            name: "Alice",
            age: 31
        })
    );
    assert_eq!(
        None,
        hm.get(&Key {
            name: "Bob",
            age: 41
        })
    );
}

struct MyStruct {
    map: MyHashMapType,
}

// Demonstrates how to embed a frozen hash map into a struct. This makes it possible to have
// multiple instances of a frozen map with common keys, but unique values.
fn use_struct() {
    let mut ms = MyStruct {
        map: MY_HASH_MAP.clone(),
    };

    // set a custom value
    if let Some(v) = ms.map.get_mut(&Key {
        name: "Alice",
        age: 30,
    }) {
        *v = 3;
    }

    assert_eq!(
        Some(&3),
        ms.map.get(&Key {
            name: "Alice",
            age: 30
        })
    );
    assert_eq!(
        Some(&2),
        ms.map.get(&Key {
            name: "Bob",
            age: 40
        })
    );

    assert_eq!(
        None,
        ms.map.get(&Key {
            name: "Alice",
            age: 31
        })
    );
    assert_eq!(
        None,
        ms.map.get(&Key {
            name: "Bob",
            age: 41
        })
    );
}

// Demonstrates how to use a facade to create a frozen hash map.
//
// Using a facade is slower than using the fz_hash_map! macro, but it
// allows keys that are determined at runtime, whereas the macro requires
// the keys to be known at compile time.
fn use_facade() {
    let hm = FzHashMap::new(vec![
        (
            Key {
                name: "Alice",
                age: 30,
            },
            1,
        ),
        (
            Key {
                name: "Bob",
                age: 40,
            },
            2,
        ),
    ]);

    assert_eq!(
        Some(&1),
        hm.get(&Key {
            name: "Alice",
            age: 30
        })
    );
    assert_eq!(
        Some(&2),
        hm.get(&Key {
            name: "Bob",
            age: 40
        })
    );

    assert_eq!(
        None,
        hm.get(&Key {
            name: "Alice",
            age: 31
        })
    );
    assert_eq!(
        None,
        hm.get(&Key {
            name: "Bob",
            age: 41
        })
    );
}

fn main() {
    use_static();
    use_struct();
    use_local();
    use_facade();
}
