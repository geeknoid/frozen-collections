use frozen_collections::*;

// The enum type we use in our example sets
#[derive(Scalar, Eq, PartialEq, Ord, PartialOrd, Clone, Copy)]
enum Color {
    Red,
    Green,
    Blue,
}

// Declare a static scalar set using an enum as a value type.
fz_scalar_set!(static MY_SCALAR_SET: MyScalarSetType<Color> = {
    Color::Red,
    Color::Green,
});

// Declare a static scalar set using integers as values.
fz_scalar_set!(static MY_INT_SET: MyIntSetType<i32> = {
    10,
    20,
});

// Demonstrates how to use a static frozen scalar set.
fn use_static() {
    assert!(MY_SCALAR_SET.contains(&Color::Red));
    assert!(MY_SCALAR_SET.contains(&Color::Green));
    assert!(!MY_SCALAR_SET.contains(&Color::Blue));

    assert!(MY_INT_SET.contains(&10));
    assert!(MY_INT_SET.contains(&20));
    assert!(!MY_INT_SET.contains(&30));
}

// Demonstrates how to create a frozen set as a local variable.
fn use_local() {
    let ss = fz_scalar_set!(Color::Red, Color::Green,);

    assert!(ss.contains(&Color::Red));
    assert!(ss.contains(&Color::Green));
    assert!(!ss.contains(&Color::Blue));
}

// Demonstrates how to use a facade to create a frozen scalar set.
//
// Using a facade is slower than using the fz_scalar_set! macro, but it
// allows values that are determined at runtime, whereas the macro requires
// the values to be known at compile time.
fn use_facade() {
    let ss = FzScalarSet::new(vec![Color::Red, Color::Green]);

    assert!(ss.contains(&Color::Red));
    assert!(ss.contains(&Color::Green));
    assert!(!ss.contains(&Color::Blue));
}

fn main() {
    use_static();
    use_local();
    use_facade();
}
