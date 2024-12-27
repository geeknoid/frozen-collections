This type requires that the keys
implement the [`Ord`] trait. This can frequently be achieved by
using `#[derive(PartialOrd, Ord)]`.

It is a logic error for a key to be modified in such a way that the key's
order, as determined by the [`Ord`] trait, or its equality, as determined by
the [`Eq`] trait, changes while it is in the map. This is normally only
possible through [`core::cell::Cell`], [`core::cell::RefCell`], global state, I/O, or unsafe code.

The behavior resulting from the above logic error can include panics, incorrect results,
memory leaks, and non-termination.

