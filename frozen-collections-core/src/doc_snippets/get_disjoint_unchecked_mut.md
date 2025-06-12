Gets multiple mutable values from the map.

# Safety

Calling this method with overlapping keys is [undefined behavior](https://doc.rust-lang.org/reference/behavior-considered-undefined.html)
even if the resulting references are not used.
