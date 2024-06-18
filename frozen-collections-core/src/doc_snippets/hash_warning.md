This type requires that the keys
implement the [`Eq`] and [`Hash`] traits. This can frequently be achieved by
using `#[derive(PartialEq, Eq, Hash)]`. If you implement these yourself,
it is important that the following property holds:

```text
k1 == k2 -> hash(k1) == hash(k2)
```

In other words, if two keys are equal, their hashes must be equal.
Violating this property is a logic error.

It is also a logic error for a key to be modified in such a way that the key's
hash, as determined by the [`Hash`] trait, or its equality, as determined by
the [`Eq`] trait, changes while it is in the collection. This is normally only
possible through [`core::cell::Cell`], [`core::cell::RefCell`], global state, I/O,
or unsafe code.

The behavior resulting from either logic error can include panics, incorrect results,
memory leaks, and non-termination.

