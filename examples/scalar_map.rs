use frozen_collections::*;

// The enum type we use to index into our example maps
#[derive(Scalar, Eq, PartialEq, Ord, PartialOrd, Clone, Copy)]
enum Color {
    Red,
    Green,
    Blue,
}

// Declare a static scalar map using an enum as a key.
fz_scalar_map!(static MY_SCALAR_MAP: MyScalarMapType<Color, i32> = {
    Color::Red: 1,
    Color::Green: 2,
});

// Declare a static scalar map using integers as keys.
fz_scalar_map!(static MY_INT_MAP: MyIntMapType<i32, i32> = {
    10: 1,
    20: 2,
});

// Demonstrates how to use a static frozen scalar map.
fn use_static() {
    assert_eq!(Some(&1), MY_SCALAR_MAP.get(&Color::Red));
    assert_eq!(Some(&2), MY_SCALAR_MAP.get(&Color::Green));
    assert_eq!(None, MY_SCALAR_MAP.get(&Color::Blue));

    assert_eq!(Some(&1), MY_INT_MAP.get(&10));
    assert_eq!(Some(&2), MY_INT_MAP.get(&20));
    assert_eq!(None, MY_INT_MAP.get(&30));
}

// Demonstrates how to create a frozen map as a local variable.
fn use_local() {
    let sm = fz_scalar_map!(
        Color::Red: 1,
        Color::Green: 2,
    );

    assert_eq!(Some(&1), sm.get(&Color::Red));
    assert_eq!(Some(&2), sm.get(&Color::Green));
    assert_eq!(None, sm.get(&Color::Blue));
}

struct MyStruct {
    map: MyScalarMapType,
}

// Demonstrates how to embed a frozen scalar map into a struct. This makes it possible to have
// multiple instances of a frozen map with common keys, but unique values.
fn use_struct() {
    let mut ms = MyStruct {
        map: MY_SCALAR_MAP.clone(),
    };

    // set a custom value
    if let Some(v) = ms.map.get_mut(&Color::Red) {
        *v = 3;
    }

    assert_eq!(Some(&3), ms.map.get(&Color::Red));
    assert_eq!(Some(&2), ms.map.get(&Color::Green));
    assert_eq!(None, ms.map.get(&Color::Blue));
}

// Demonstrates how to use a facade to create a frozen scalar map.
//
// Using a facade is slower than using the fz_scalar_map! macro, but it
// allows keys that are determined at runtime, whereas the macro requires
// the keys to be known at compile time.
fn use_facade() {
    let sm = FzScalarMap::new(vec![(Color::Red, 1), (Color::Green, 2)]);

    assert_eq!(Some(&1), sm.get(&Color::Red));
    assert_eq!(Some(&2), sm.get(&Color::Green));
    assert_eq!(None, sm.get(&Color::Blue));
}

fn main() {
    use_static();
    use_struct();
    use_local();
    use_facade();
}
